#!/usr/bin/env bash
set -e

# Move to working dir
script_dir="$(cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)"
cd "$script_dir/.."

lib_dir="$(realpath ./mqtt-lib)"
mkdir -p $lib_dir
cd $lib_dir

if [[ ! -d paho.mqtt.c ]]
then
    git clone https://github.com/eclipse/paho.mqtt.c.git
fi

# Build and install
mkdir -p paho.mqtt.c/build
cd paho.mqtt.c/build

cmake -DCMAKE_C_FLAGS=-pg -DCMAKE_BUILD_TYPE=RelWithDebInfo -DCMAKE_INSTALL_PREFIX="$lib_dir/mosquitto-install" ..
make
make install
