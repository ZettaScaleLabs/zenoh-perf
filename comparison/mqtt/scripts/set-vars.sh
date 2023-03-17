#!/bin/false

src_dir="${1:-$(pwd)}"
lib_dir="$(realpath $src_dir/mqtt-lib/mosquitto-install)"
export CPATH="$lib_dir/include:$CPATH"
export LIBRARY_PATH="$lib_dir/lib:$LIBRARY_PATH"
export LD_LIBRARY_PATH="$lib_dir/lib:$LD_LIBRARY_PATH"
