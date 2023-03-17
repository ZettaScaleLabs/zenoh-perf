mod opts;

use anyhow::{anyhow, ensure, Result};
use async_std::{sync::Arc, task};
use clap::Parser;
use kafka_test::{AsyncStdStreamConsumer, DEFAULT_GROUP_ID};
use log::{error, info, trace};
use opts::Opts;
use rdkafka::{
    consumer::Consumer, error::KafkaError, types::RDKafkaErrorCode, ClientConfig, Message as _,
};
use std::{
    process,
    sync::atomic::{AtomicUsize, Ordering},
    time::{Duration, Instant},
};

#[async_std::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let opts = Opts::parse();
    let timeout = opts.timeout;

    let future = run_consumer(opts);
    if let Some(timeout) = timeout {
        async_std::future::timeout(timeout, future)
            .await
            .map_err(|_| anyhow!("timeout"))??;
    } else {
        future.await?;
    }

    Ok(())
}

async fn run_consumer(opts: Opts) -> Result<()> {
    use KafkaError as E;
    use RDKafkaErrorCode as C;

    let consumer_id = process::id();
    info!(
        "Start consumer {} on topic {} and group-id {}",
        consumer_id, opts.topic, DEFAULT_GROUP_ID
    );

    // Configure the consumer
    let create_consumer = || -> Result<_> {
        let consumer: AsyncStdStreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &opts.brokers)
            .set("group.id", DEFAULT_GROUP_ID)
            .set("session.timeout.ms", "6000")
            .create()?;
        consumer.subscribe(&[&opts.topic])?;
        Ok(consumer)
    };
    let mut consumer = create_consumer()?;
    let counter = Arc::new(AtomicUsize::new(0));

    async_std::task::spawn(measure(counter.clone(), opts.payload_size));

    loop {
        let result = consumer.recv().await;
        let msg = match result {
            Ok(msg) => msg,
            Err(E::MessageConsumption(C::UnknownTopicOrPartition)) => {
                trace!("The topic {} is not created yet, retry again", opts.topic);
                continue;
            }
            Err(err) => return Err(err.into()),
        };

        let payload = match msg.payload() {
            Some(payload) => payload,
            None => {
                error!("Ignore a message without payload");
                consumer = create_consumer()?;
                continue;
            }
        };

        // decode the producer ID and message index in the payload
        let info = match parse_payload(payload, opts.payload_size) {
            Ok(info) => info,
            Err(err) => {
                error!("Unable to parse payload: {:?}", err);
                continue;
            }
        };
        trace!(
            "Consumer {} receives a payload with index {} and size {} from producer {}",
            consumer_id,
            info.msg_index,
            payload.len(),
            info.producer_id
        );
        counter.fetch_add(1, Ordering::Relaxed);
    }
}

pub fn parse_payload(payload: &[u8], expect_size: usize) -> Result<PayloadInfo> {
    let payload_size = payload.len();
    ensure!(
        payload_size >= 8,
        "insufficient payload size {}",
        payload_size
    );
    ensure!(
        payload_size == expect_size,
        "payload size does not match, expect {} bytes, but received {} bytes",
        expect_size,
        payload_size
    );

    let producer_id = u32::from_le_bytes(payload[0..4].try_into().unwrap());
    let msg_index = u32::from_le_bytes(payload[4..8].try_into().unwrap());

    Ok(PayloadInfo {
        producer_id,
        msg_index,
        payload_size,
    })
}

#[derive(Debug, Clone)]
pub struct PayloadInfo {
    pub producer_id: u32,
    pub msg_index: u32,
    pub payload_size: usize,
}

async fn measure(messages: Arc<AtomicUsize>, payload: usize) {
    let mut timer = Instant::now();
    loop {
        task::sleep(Duration::from_secs(1)).await;

        if messages.load(Ordering::Relaxed) > 0 {
            let elapsed = timer.elapsed().as_micros() as f64;
            let c = messages.swap(0, Ordering::Relaxed);
            println!("{},{:.3}", payload, c as f64 * 1_000_000.0 / elapsed);
            timer = Instant::now()
        }
    }
}
