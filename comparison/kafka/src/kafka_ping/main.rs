mod opts;

use anyhow::{anyhow, ensure, Context, Result};
use clap::Parser;
use kafka_test::{AsyncStdFutureProducer, AsyncStdStreamConsumer, DEFAULT_GROUP_ID};
use log::{error, info, trace};
use once_cell::sync::Lazy;
use opts::Opts;
use rdkafka::{
    consumer::Consumer, error::KafkaError, producer::FutureRecord, types::RDKafkaErrorCode,
    ClientConfig, Message,
};
use std::{
    process,
    time::{Duration, SystemTime},
};

static SINCE: Lazy<SystemTime> = Lazy::new(SystemTime::now);
const MIN_PAYLOAD_SIZE: usize = 16;

struct PayloadInfo {
    pub ping_id: u32,
    pub msg_idx: u32,
    pub rtt: Duration,
}

#[async_std::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let opts = Opts::parse();
    let timeout = opts.timeout;

    let future = run_latency_benchmark(opts);
    if let Some(timeout) = timeout {
        async_std::future::timeout(timeout, future)
            .await
            .map_err(|_| anyhow!("timeout"))??;
    } else {
        future.await?;
    }

    Ok(())
}

async fn run_latency_benchmark(opts: Opts) -> Result<()> {
    let ping_id = process::id();
    let client_config = {
        let mut conf = ClientConfig::new();
        conf.set("bootstrap.servers", &opts.brokers);
        conf
    };
    info!("Start ping {}", ping_id);

    run_ping_pong(&opts, &client_config, ping_id).await?;

    Ok(())
}

fn generate_payload(size: usize, ping_id: u32, msg_idx: u32) -> Vec<u8> {
    assert!(
        size >= MIN_PAYLOAD_SIZE,
        "The minimum payload size is {} bytes",
        MIN_PAYLOAD_SIZE
    );
    let since = *SINCE; // the SINCE is inited earlier than the next SystemTime::now()
    let dur = SystemTime::now().duration_since(since).unwrap();
    let micros = dur.as_micros() as u64;

    let ping_id_bytes = ping_id.to_le_bytes();
    let msg_idx_bytes = msg_idx.to_le_bytes();
    let time_bytes = micros.to_le_bytes();

    let mut payload = vec![0u8; size];
    payload[0..4].copy_from_slice(&ping_id_bytes);
    payload[4..8].copy_from_slice(&msg_idx_bytes);
    payload[8..16].copy_from_slice(&time_bytes);
    payload
}

fn parse_payload(payload: &[u8], expect_payload_size: usize) -> Result<PayloadInfo> {
    let payload_size = payload.len();
    ensure!(
        payload_size >= MIN_PAYLOAD_SIZE,
        "The payload size ({} bytes) is less than the required minimum {} bytes",
        payload_size,
        MIN_PAYLOAD_SIZE
    );

    let ping_id_bytes = &payload[0..4];
    let msg_idx_bytes = &payload[4..8];
    let time_bytes = &payload[8..16];

    let micros = u64::from_le_bytes(time_bytes.try_into().unwrap());
    let since = *SINCE + Duration::from_micros(micros);
    let rtt = SystemTime::now()
        .duration_since(since)
        .with_context(|| "the timestamp goes backward")?;

    let ping_id = u32::from_le_bytes(ping_id_bytes.try_into().unwrap());
    let msg_idx = u32::from_le_bytes(msg_idx_bytes.try_into().unwrap());

    ensure!(
        payload.len() == expect_payload_size,
        "Expect payload size to be {} bytes, but get {} bytes",
        expect_payload_size,
        payload_size
    );

    Ok(PayloadInfo {
        msg_idx,
        rtt,
        ping_id,
    })
}

fn create_producer(opts: &Opts, mut config: ClientConfig) -> Result<AsyncStdFutureProducer> {
    if let Some(cfgs) = &opts.producer_configs {
        cfgs.iter().for_each(|kv| {
            config.set(&kv.key, &kv.val);
        });
    }

    let producer: AsyncStdFutureProducer = config.create()?;
    Ok(producer)
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

async fn run_ping_pong(opts: &Opts, client_config: &ClientConfig, ping_id: u32) -> Result<()> {
    let producer: AsyncStdFutureProducer = create_producer(opts, client_config.clone())?;
    let mut consumer = create_consumer(opts, client_config.clone(), &opts.pong_topic)?;

    for count in 0.. {
        send(opts, &producer, ping_id, count).await?;
        if !recv(opts, client_config, ping_id, &mut consumer).await? {
            panic!("Failed to receive pong message.");
        }
        async_std::task::sleep(Duration::from_secs_f64(opts.interval)).await;
    }
    Ok(())
}

async fn send(
    opts: &Opts,
    producer: &AsyncStdFutureProducer,
    ping_id: u32,
    msg_idx: u32,
) -> Result<()> {
    let record_key = ping_id.to_le_bytes();
    let payload = generate_payload(opts.payload_size, ping_id, msg_idx);
    let record = FutureRecord::to(&opts.ping_topic)
        .payload(&payload)
        .key(&record_key);

    trace!(
        "Send a ping with ping_id {} and msg_idx {}",
        ping_id,
        msg_idx
    );
    producer
        .send(record, Duration::ZERO)
        .await
        .map_err(|(err, _msg)| err)?;
    Ok(())
}

async fn recv(
    opts: &Opts,
    client_config: &ClientConfig,
    ping_id: u32,
    consumer: &mut AsyncStdStreamConsumer,
) -> Result<bool> {
    use KafkaError as E;
    use RDKafkaErrorCode as C;

    const RECV_TIMEOUT: Duration = Duration::from_secs(1);

    let recv = consumer.recv();
    let result = async_std::future::timeout(RECV_TIMEOUT, recv).await;

    match result {
        Ok(Ok(msg)) => {
            let payload = match msg.payload() {
                Some(payload) => payload,
                None => {
                    error!("Ignore a message without payload");
                    return Ok(false);
                }
            };

            let info = match parse_payload(payload, opts.payload_size) {
                Ok(info) => info,
                Err(err) => {
                    error!("Unable to parse payload: {:#}", err);
                    return Ok(false);
                }
            };

            trace!(
                "Received a pong with ping_id {} and msg_idx {}",
                info.ping_id,
                info.msg_idx
            );
            ensure!(
                info.ping_id == ping_id,
                "Ignore the payload from a foreign ping ID {}",
                info.ping_id
            );

            println!("{},{}", opts.interval, info.rtt.as_micros() / 2);

            Ok(true)
        }
        Ok(Err(E::MessageConsumption(C::UnknownTopicOrPartition))) => {
            trace!(
                "The topic {} is not created yet, retry again",
                opts.pong_topic
            );
            async_std::task::sleep(Duration::from_secs(1)).await;
            *consumer = create_consumer(opts, client_config.clone(), &opts.pong_topic)?;
            Ok(true)
        }
        Ok(Err(err)) => Err(err.into()),
        Err(_) => {
            trace!("Timeout receiving a message");
            Ok(true)
        }
    }
}
