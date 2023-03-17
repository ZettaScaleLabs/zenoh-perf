mod opts;

use anyhow::{anyhow, Result};
use clap::Parser;
use kafka_test::AsyncStdFutureProducer;
use log::{info, trace};
use opts::Opts;
use rdkafka::{producer::FutureRecord, ClientConfig};
use std::{process, time::Duration};

#[async_std::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let opts = Opts::parse();
    let timeout = opts.timeout;

    let future = run_producer(opts);
    if let Some(timeout) = timeout {
        async_std::future::timeout(timeout, future)
            .await
            .map_err(|_| anyhow!("timeout"))??;
    } else {
        future.await?;
    }

    Ok(())
}

async fn run_producer(opts: Opts) -> Result<()> {
    let producer_id = process::id();
    info!("Start producer {} on topic {}", producer_id, opts.topic);

    // Configure the producer
    let mut client_config = ClientConfig::new();
    client_config.set("bootstrap.servers", &opts.brokers);

    if let Some(cfgs) = opts.producer_configs {
        cfgs.iter().for_each(|kv| {
            client_config.set(&kv.key, &kv.val);
        });
    }

    let producer: AsyncStdFutureProducer = client_config.create()?;
    let payload: Vec<u8> = (0..opts.payload_size).map(|i| (i % 10) as u8).collect();

    for msg_idx in 0.. {
        let key = producer_id.to_le_bytes();
        let record = FutureRecord::to(&opts.topic).payload(&payload).key(&key);

        trace!("Producer {} sends a message {}", producer_id, msg_idx);
        producer
            .send(record, Duration::ZERO)
            .await
            .map_err(|(err, _msg)| err)?;
    }

    Ok(())
}
