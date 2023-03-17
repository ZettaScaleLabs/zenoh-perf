#!/usr/bin/env bash
set -e

function print_usage() {
    echo "Usage: $0 PAYLOAD_SIZE LOG_DIR [TIMEOUT]"
    exit 1
}

payload_size="$1"
shift || print_usage

log_dir="$1"
shift || print_usage

if [[ -n "$1" ]]
then
    timeout_arg="--timeout $1"
    shift
fi

rm -rf "$log_dir"

cargo build --release --all-targets


! {
    echo "cargo run --bin kafka_ping --release -- -p $payload_size -P linger.ms=0 -P batch.size=1 -P compression.type=none -C fetch.min.bytes=1"
    echo "cargo run --bin kafka_pong --release -- -P linger.ms=0 -P batch.size=1 -P compression.type=none -C fetch.min.bytes=1"
} | parallel --jobs 0 --halt now,fail=1 $timeout_arg --results "${log_dir}/{#}_{/.}"
