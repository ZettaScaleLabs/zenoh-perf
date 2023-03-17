#!/usr/bin/env bash

# Move to working dir
script_dir="$(cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)"
cd "$script_dir/.."

VERSION=2.0.15
wget -nc -c https://github.com/eclipse/mosquitto/archive/refs/tags/v${VERSION}.tar.gz

tar xvf v${VERSION}.tar.gz 
mv mosquitto-${VERSION} mosquitto
cd mosquitto
make -j binary WITH_CJSON=no
