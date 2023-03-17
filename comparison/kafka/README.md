# User Guide

## Build the source

```bash
make
```

## Run Throughput Test

**Launch Kafka Zookeeper and Server**
```bash
./scripts/run_kafka_single.sh BROKER_IP
```

**Pub**
```bash
KAFKA_PUB_THR_OPTIONS="-P message.timeout.ms=5000 \
-P linger.ms=0 \
-P batch.size=400KB \
-P compression.type=none \
-P acks=0" \
../../target/release/kafka_pub_thr \
$KAFKA_PUB_THR_OPTIONS \
-b BROKER_IP:9092 \
-p PAYLOAD_SIZE_IN_BYTES
```

**Sub**
```bash
KAFKA_SUB_THR_OPTIONS="--warmup-msgs 0" \
../../target/release/kafka_sub_thr \
$KAFKA_SUB_THR_OPTIONS \
-p PAYLOAD_SIZE_IN_BYTES \
-b BROKER_IP:9092
```


The results is formatted as

```bash
payload_size_in_bytes, throughput_in_message_per_second
```

<details>

<summary>
Click me to see an example.
</summary>

> #### Launch a throughput test with payload size 8 bytes through loopback
>
> **Launch Kafka Zookeeper and Server**
> ```bash
> ./scripts/run_kafka_single.sh 127.0.0.1
> ```
>
> **Pub**
> ```bash
> KAFKA_PUB_THR_OPTIONS="-P message.timeout.ms=5000 \
> -P linger.ms=0 \
> -P batch.size=400KB \
> -P compression.type=none \
> -P acks=0" \
> ../../target/release/kafka_pub_thr \
> $KAFKA_PUB_THR_OPTIONS \
> -b 127.0.0.1:9092 \
> -p 8
> ```
>
> **Sub**
> ```bash
> KAFKA_SUB_THR_OPTIONS="--warmup-msgs 0" \
> ../../target/release/kafka_sub_thr \
> $KAFKA_SUB_THR_OPTIONS \
> -p 8 \
> -b 127.0.0.1:9092
> ```
>
> Then we get
>
> ```txt
> 8,193.994
> 8,194.996
> 8,191.994
> 8,193.994
> 8,193.991
> ```

</details>

## Run Latency Test

**Launch Kafka Zookeeper and Server**
```bash
./scripts/run_kafka_single.sh BROKER_IP
```

**Pong**
```bash
KAFKA_PONG_OPTIONS=" -P linger.ms=0 \
-P batch.size=1 \
-P compression.type=none \
-P acks=0 \
-C fetch.min.bytes=1" \
../../target/release/kafka_pong \
$KAFKA_PONG_OPTIONS \
-b BROKER_IP:9092
```

**Ping**
```bash
KAFKA_PING_OPTIONS=" -P linger.ms=0 \
-P batch.size=1 \
-P compression.type=none \
-P acks=0 \
-C fetch.min.bytes=1" \
../../target/release/kafka_ping \
$KAFKA_PING_OPTIONS \
-i INTERVAL_IN_SECONDS \
-p PAYLOAD_SIZE_IN_BYTES \
-b BROKER_IP:9092
```

The results is formatted as

```bash
INTERVAL_IN_SECONDS, LATENCY_IN_MICRO_SECONDS
```

<details>
<summary>Click me to see an example.</summary>

> #### Launch a brokered latency test with payload size 64 bytes and interval 0.1 second through loopback
>
> **Launch Kafka Zookeeper and Server**
> ```bash
> ./scripts/run_kafka_single.sh 127.0.0.1
> ```
>
> **Pong**
> ```bash
> KAFKA_PONG_OPTIONS=" -P linger.ms=0 \
> -P batch.size=1 \
> -P compression.type=none \
> -P acks=0 \
> -C fetch.min.bytes=1" \
> ../../target/release/kafka_pong \
> $KAFKA_PONG_OPTIONS \
> -b 127.0.0.1:9092
> ```
>
> **Ping**
> ```bash
> KAFKA_PING_OPTIONS=" -P linger.ms=0 \
> -P batch.size=1 \
> -P compression.type=none \
> -P acks=0 \
> -C fetch.min.bytes=1" \
> ../../target/release/kafka_ping \
> $KAFKA_PING_OPTIONS \
> -i 0.1 \
> -p 64 \
> -b 127.0.0.1:9092
> ```
>
> Then we get
> ```txt
> 0.1,5496
> 0.1,8286
> 0.1,5433
> 0.1,5440
> 0.1,5461
> ```

</details>
