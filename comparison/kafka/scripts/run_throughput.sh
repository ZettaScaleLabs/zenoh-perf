#!/usr/bin/env bash
set -e

function print_usage() {
    echo "Usage: $0 PAYLOAD_SIZE N_MSGS_PER_MEASAURE NUM_PUBLISHERS LOG_DIR [TIMEOUT]"
    exit 1
}

payload_size="$1"
shift || print_usage

n_msgs_per_measure="$1"
shift || print_usage

num_publishers="$1"
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
    echo "cargo run --bin kafka_sub_thr --release -- -p $payload_size -n $n_msgs_per_measure -s 0"
    
    for ((c=0; c<$num_publishers; c+=1))
    do
        echo "cargo flamegraph --bin kafka_pub_thr -- -p $payload_size -P linger.ms=0 -P batch.size=400KB -P compression.type=none -P acks=0"
    done
} | parallel --jobs 0 --halt now,fail=1 $timeout_arg --line-buffer # --results "${log_dir}/{#}_{/.}"
