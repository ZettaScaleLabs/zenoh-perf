#!/usr/bin/env bash
set -e

function print_usage {
    echo "Usage: $0 SERVER_ADDRESS" >&2
}

# Move to working dir
script_dir="$(cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)"
if [ ! -d "${script_dir}/files/kafka-pkg" ]; then
    ${script_dir}/install-kafka.sh
fi
cd "${script_dir}/files/kafka-pkg"

ADDRESS="$1"
shift || {
    print_usage;
    exit 1;
}

# Write config files
export ADDRESS="$ADDRESS"
export KAFKA_LOG_DIR="${script_dir}/files/cache-single/kafka-logs"
export ZOOKEEPER_DATA_DIR="${script_dir}/files/cache-single/zookeeper-data"

mkdir -p "$KAFKA_LOG_DIR"
mkdir -p "$ZOOKEEPER_DATA_DIR"

envsubst < "${script_dir}/files/config-single/server.properties" > config/server.properties
envsubst < "${script_dir}/files/config-single/zookeeper.properties" > config/zookeeper.properties

# Clean up data from previous session
"${script_dir}/clean_kafka.sh"

# Run servers
(
    echo 'bin/zookeeper-server-start.sh config/zookeeper.properties'
    echo 'bin/kafka-server-start.sh config/server.properties'
) | parallel --max-procs 0
