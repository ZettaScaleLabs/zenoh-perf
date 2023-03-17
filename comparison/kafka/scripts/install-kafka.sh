#!/usr/bin/env bash
set -e

script_dir="$(cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)"
mkdir -p "$script_dir/files"
cd "$script_dir/files"

# Download Kafka
KAFKA_VER=3.3.1
SCALA_VER=2.13

# Since 3.3.1 has been archived
# wget --no-clobber "https://downloads.apache.org/kafka/${KAFKA_VER}/kafka_${SCALA_VER}-${KAFKA_VER}.tgz"
wget --no-clobber "https://archive.apache.org/dist/kafka/${KAFKA_VER}/kafka_${SCALA_VER}-${KAFKA_VER}.tgz"

tar --overwrite -xf kafka_${SCALA_VER}-${KAFKA_VER}.tgz
rm -rf kafka-pkg
mv kafka_${SCALA_VER}-${KAFKA_VER} kafka-pkg
