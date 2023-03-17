#!/usr/bin/env bash
set -e

# Move to working dir
script_dir="$(cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)"
cd "$script_dir/.."

groups | grep -q docker || {
    echo \"Warning: You may not have permission to execute docker. Try this command:\" >&2
    echo \"         usermod -a -G docker YOUR_USERNAME\" >&2
}

VERSION=${1:-2.0.15}
CONFIG=${2:-$script_dir/../mosquitto.conf}
PORT=${3:-1883}
CPUS=${4:-0,1}

docker run \
    --init \
    --cpuset-cpus $CPUS \
    -p $PORT:1883 \
    --rm \
    -v $CONFIG:/mosquitto/config/mosquitto.conf \
    eclipse-mosquitto:$VERSION
