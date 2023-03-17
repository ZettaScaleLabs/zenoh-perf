#!/usr/bin/env bash
set -e

n_msgs_per_measure=128

script_dir="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
base_log_dir="${script_dir}/$(date --rfc-3339=seconds | tr ' ' 'T' | tr ':' '-')_throughput"

for payload_size in $(cat "$script_dir/payload_size.txt")
do
    for num_pub in 1 2 3 4 5 6 7 8
    do
        echo "Running with payload_size=$payload_size num_pub=$num_pub"
        log_dir="${base_log_dir}/payload-${payload_size}_num-pub-${num_pub}"
        "$script_dir/run_throughput.sh" "$payload_size" "$n_msgs_per_measure" "$num_pub" "$log_dir" 10s
    done
done
