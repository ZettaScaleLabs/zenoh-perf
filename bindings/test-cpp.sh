#!/usr/bin/env bash

LOG_DIR="_logs/zenoh-cpp"
TIMEOUT=10s
rm -rf $LOG_DIR &> /dev/null
mkdir -p $LOG_DIR

export ASYNC_STD_THREAD_COUNT=1
export ZENOH_RUNTIME_THREADS="tx=1;rx=1;net=1;acceptor=1;application=1"

base="zenohc"

# This doesn't work
# base="zenohpico"

LOG_FILE=${LOG_DIR}/latency.log
rm $LOG_FILE &> /dev/null
mkfifo /tmp/srv-input &> /dev/null
parallel --halt now,success=1 --lb <<EOL
taskset -c 0,2 ./zenoh-cpp/build/${base}/zenoh_pong -c ./config/peer-listen.json5
sleep 1 && taskset -c 1,3 ./zenoh-cpp/build/${base}/zenoh_ping -c ./config/peer-connect.json5 -s 64 -i 0 > $LOG_FILE
sleep 1 && sleep $TIMEOUT
EOL
pkill zenoh_ping
pkill zenoh_pong
