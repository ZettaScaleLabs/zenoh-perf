#!/usr/bin/env bash

LOG_DIR="_logs/zenoh-cpp"
TIMEOUT=10s
rm -rf $LOG_DIR &> /dev/null
mkdir -p $LOG_DIR

for branch in "main" "tokio"; do
    LOG_FILE=${LOG_DIR}/${branch}.log
    rm $LOG_FILE &> /dev/null
    mkfifo /tmp/srv-input &> /dev/null
    parallel --halt now,success=1 --lb <<EOL
tail -f /tmp/srv-input | taskset -c 0,2 ./zenoh-c/build-${branch}/zenoh_pong
sleep 1 && taskset -c 1,3 ./zenoh-c/build-${branch}/zenoh_ping -s 64 -i 0 > $LOG_FILE
sleep 1 && sleep $TIMEOUT
EOL
done
