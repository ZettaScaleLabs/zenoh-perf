use anyhow::Result;
use clap::{Parser, ValueEnum};
use kafka_test::DEFAULT_THROUGHPUT_TOPIC;
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

    #[clap(long, default_value = "0")]
    pub warmup_msgs: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
#[clap(rename_all = "snake_case")]
pub enum Mode {
    Peer,
    Client,
}

fn parse_duration(text: &str) -> Result<Duration> {
    let dur = humantime::parse_duration(text)?;
    Ok(dur)
}
