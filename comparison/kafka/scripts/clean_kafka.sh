#!/usr/bin/env bash
set -e

# Move to working dir
script_dir="$(cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)"
cd "$script_dir"

# Kill Kafka and Zookeeper servers
killall java >/dev/null 2>&1 || true

# Remove cache files
rm -rf files/cache-single files/cache-cluster
