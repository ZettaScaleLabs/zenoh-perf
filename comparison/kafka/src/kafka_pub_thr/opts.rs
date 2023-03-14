use anyhow::Result;
use clap::Parser;
use kafka_test::{KeyVal, DEFAULT_THROUGHPUT_TOPIC};
use std::time::Duration;

#[derive(Parser)]
pub struct Opts {
    #[clap(long, default_value_t = DEFAULT_THROUGHPUT_TOPIC.to_string())]
    pub topic: String,
    #[clap(long, parse(try_from_str = parse_duration))]
    pub timeout: Option<Duration>,
    #[clap(short = 'b', long, default_value = "127.0.0.1")]
    pub brokers: String,
    #[clap(short = 'p', long)]
    pub payload_size: usize,
    #[clap(short = 'P', long)]
    pub producer_configs: Option<Vec<KeyVal>>,
}

fn parse_duration(text: &str) -> Result<Duration> {
    let dur = humantime::parse_duration(text)?;
    Ok(dur)
}
