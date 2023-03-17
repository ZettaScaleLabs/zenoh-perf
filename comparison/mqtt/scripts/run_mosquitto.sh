#!/usr/bin/env bash
set -e

# Move to working dir
script_dir="$(cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)"
cd "$script_dir/../mosquitto"

VERSION=${1:-2.0.15}
CONFIG=${2:-$script_dir/../mosquitto.conf}
PORT=${3:-1883}
CPUS=${4:-0,1}

taskset -a -c $CPUS ./src/mosquitto -c $CONFIG -p $PORT
