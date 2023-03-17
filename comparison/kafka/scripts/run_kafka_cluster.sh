#!/usr/bin/env bash
set -e

# Configuration
export MY_ID=3
export ADDRESS=10.8.0.253
export SERVER1=10.8.0.251
export SERVER2=10.8.0.252
export SERVER3=10.8.0.253

# Move to working dir
script_dir="$(cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)"
cd "${script_dir}/files/kafka_2.13-3.2.1"

# Write config files
export KAFKA_LOG_DIR="${script_dir}/files/cache-cluster/kafka-logs"
export ZOOKEEPER_DATA_DIR="${script_dir}/files/cache-cluster/zookeeper-data"

mkdir -p "$KAFKA_LOG_DIR"
mkdir -p "$ZOOKEEPER_DATA_DIR"
echo "$MY_ID" > "${ZOOKEEPER_DATA_DIR}/myid"

envsubst < "${script_dir}/files/config-cluster/server.properties" > config/server.properties
envsubst < "${script_dir}/files/config-cluster/zookeeper.properties" > config/zookeeper.properties

# Clean up data from previous session
"${script_dir}/clean_kafka.sh"

# Run servers
(
    echo 'bin/zookeeper-server-start.sh config/zookeeper.properties'
    echo 'bin/kafka-server-start.sh config/server.properties'
) | parallel --max-procs 0
