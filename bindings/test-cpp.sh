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

for branch in "main" "tokio"; do
    LOG_FILE=${LOG_DIR}/${branch}.log
    rm $LOG_FILE &> /dev/null
    mkfifo /tmp/srv-input &> /dev/null
    parallel --halt now,success=1 --lb <<EOL
tail -f /tmp/srv-input | taskset -c 0,2 ./zenoh-cpp/build-${branch}/${base}/z_pong
sleep 1 && taskset -c 1,3 ./zenoh-cpp/build-${branch}/${base}/z_ping -s 64 -i 0 > $LOG_FILE
sleep 1 && sleep $TIMEOUT
EOL
    pkill z_ping
    pkill z_pong
done
