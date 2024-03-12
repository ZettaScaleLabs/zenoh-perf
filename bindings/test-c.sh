#!/usr/bin/env bash

LOG_DIR="_logs/zenoh-c"
TIMEOUT=10s
rm -rf $LOG_DIR &> /dev/null
mkdir -p $LOG_DIR

export ZENOH_RUNTIME_THREADS="tx=1;rx=1;net=1;acceptor=1;application=1"

for branch in "main" "tokio"; do
    LOG_FILE=${LOG_DIR}/${branch}.log
    rm $LOG_FILE &> /dev/null
    parallel --halt now,success=1 --lb <<EOL
taskset -c 0,2 ./zenoh-c/build-${branch}/zenoh_pong -c ./config/peer-listen.json5
sleep 1 && taskset -c 1,3 ./zenoh-c/build-${branch}/zenoh_ping -c ./config/peer-connect.json5 -s 64 -i 0 > $LOG_FILE
sleep 1 && sleep $TIMEOUT
EOL
    pkill zenoh_ping
    pkill zenoh_pong
done
