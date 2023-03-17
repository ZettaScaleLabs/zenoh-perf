mod opts;

use anyhow::{anyhow, Result};
use clap::Parser;
use kafka_test::{AsyncStdFutureProducer, AsyncStdStreamConsumer, DEFAULT_GROUP_ID};
use log::{info, trace, warn};
use opts::Opts;
use rdkafka::{
    consumer::Consumer, error::KafkaError, producer::FutureRecord, types::RDKafkaErrorCode,
    ClientConfig, Message as _,
};
use std::{process, time::Duration};

#[async_std::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let opts = Opts::parse();
    let timeout = opts.timeout;
    let future = run_pong(opts);

    if let Some(timeout) = timeout {
        async_std::future::timeout(timeout, future)
            .await
            .map_err(|_| anyhow!("timeout"))??;
    } else {
        future.await?;
    }

    Ok(())
}

async fn run_pong(opts: Opts) -> Result<()> {
    let pong_id = process::id();
    info!("Start pong {}", pong_id);

    let client_config = {
        let mut conf = ClientConfig::new();
        conf.set("bootstrap.servers", &opts.brokers);
        conf
    };

    // Configure the consumer
    let mut consumer: AsyncStdStreamConsumer =
        create_consumer(&opts, client_config.clone(), &opts.ping_topic)?;

    let record_key = pong_id.to_le_bytes();
    let mut client_config = client_config.clone();
    if let Some(cfgs) = &opts.producer_configs {
        cfgs.iter().for_each(|kv| {
            client_config.set(&kv.key, &kv.val);
        });
    }

    let producer: AsyncStdFutureProducer = client_config.create()?;

    use KafkaError as E;
    use RDKafkaErrorCode as C;

    loop {
        let msg = loop {
            let result = consumer.recv().await;
            match result {
                Ok(msg) => break msg.detach(),
                Err(E::MessageConsumption(C::UnknownTopicOrPartition)) => {
                    // retry
                    trace!(
                        "The topic {} is not created yet, retry again",
                        &opts.ping_topic
                    );
                    async_std::task::sleep(Duration::from_secs(1)).await;
                    consumer = create_consumer(&opts, client_config.clone(), &opts.ping_topic)?;
                    continue;
                }
                Err(err) => return Err(err.into()),
            }
        };

        trace!("received a ping");

        let payload = match msg.payload() {
            Some(payload) => payload,
            None => {
                warn!("the payload does not present in the received message");
                continue;
            }
        };

        let record = FutureRecord::to(&opts.pong_topic)
            .payload(payload)
            .key(&record_key);

        trace!("send a pong");
        producer
            .send(record, Duration::ZERO)
            .await
            .map_err(|(err, _msg)| err)?;
    }
}

fn create_consumer(
    opts: &Opts,
    mut config: ClientConfig,
    topic: &str,
) -> Result<AsyncStdStreamConsumer> {
    config
        .set("group.id", DEFAULT_GROUP_ID)
        .set("session.timeout.ms", "6000");

    if let Some(configs) = &opts.consumer_configs {
        for kv in configs {
            config.set(&kv.key, &kv.val);
        }
    }

    let consumer: AsyncStdStreamConsumer = config.create()?;
    consumer.subscribe(&[topic])?;
    Ok(consumer)
}
