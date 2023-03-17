#!/usr/bin/env bash
set -e

interval=1                      # in seconds

script_dir="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
base_log_dir="${script_dir}//$(date --rfc-3339=seconds | tr ' ' 'T' | tr ':' '-')_latency"

for payload_size in $(cat "$script_dir/payload_size.txt")
do
    for interval in 1.0 0.1 0.001
    do
        echo "Running with payload_size=$payload_size interval=${interval}"
        log_dir="${base_log_dir}/payload-${payload_size}_interval-${interval}"
        "$script_dir/run_latency.sh" "$payload_size" "$interval" "$log_dir" 10s
    done
done
