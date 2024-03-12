#!/usr/bin/env bash

LOG_DIR="_logs/zenoh-c"
TIMEOUT=20s
rm -rf $LOG_DIR &> /dev/null
mkdir -p $LOG_DIR

################
# Latency Test #
################

export ASYNC_STD_THREAD_COUNT=1
export ZENOH_RUNTIME_THREADS="tx=1;rx=1;net=1;acceptor=1;application=1"

LOG_FILE=${LOG_DIR}/latency.log
rm $LOG_FILE &> /dev/null
parallel --halt now,success=1 --lb <<EOL
taskset -c 0,2 ./zenoh-c/build/zenoh_pong -c ./config/peer-listen-tx1.json5
sleep 1 && taskset -c 1,3 ./zenoh-c/build/zenoh_ping -c ./config/peer-connect-tx1.json5 -s 64 -i 0 > $LOG_FILE
sleep 1 && sleep $TIMEOUT
EOL
<<<<<<< HEAD
pkill zenoh_ping
pkill zenoh_pong

unset ASYNC_STD_THREAD_COUNT
unset ZENOH_RUNTIME_THREADS


###################
# Throughput Test #
###################

rm ${LOG_DIR}/throughput.log &> /dev/null

function run_throughput() {
    PAYLOAD=$1

    parallel --halt now,success=1 --lb <<EOL
taskset -c 0,2 ./zenoh-c/build/zenoh_sub_thr -c ./config/peer-listen.json5 -s $PAYLOAD >> ${LOG_DIR}/throughput.log
sleep 1 && taskset -c 1,3 ./zenoh-c/build/zenoh_pub_thr -c ./config/peer-connect.json5 -s $PAYLOAD
sleep 1 && sleep $TIMEOUT
EOL
    pkill zenoh_sub_thr
    pkill zenoh_pub_thr
}


# for PAYLOAD in 8 16; do
# for PAYLOAD in 8 16 32 64 128 256 512 1024 2048 4096 8192 16384 32768; do
for PAYLOAD in 8 32 128 512 2048 8192 32768 131072 524288 2097152 8388608 33554432; do
    run_throughput $PAYLOAD
=======
    pkill zenoh_ping
    pkill zenoh_pong
>>>>>>> f7a1eee (Add pkill)
done
