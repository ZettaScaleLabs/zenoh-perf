# User Guide

## Build the source

```bash
make
```

## Run Throughput Test

**Pub**
```
HOST_IP=PUB_IP \
IFACE=PUB_IFACE \
CYCLONEDDS_URI=file://$(pwd)/cyclonedds.xml \
    ./throughput/target/cdds_pub -p PAYLOAD_SIZE_IN_BYTES
```

**Sub**
```
HOST_IP=SUB_IP \
IFACE=SUB_IFACE \
CYCLONEDDS_URI=file://$(pwd)/cyclonedds.xml \
    ./throughput/target/cdds_sub
```

The results is formatted as

```bash
payload_size_in_bytes, throughput_in_message_per_second
```

<details>
<summary>Click me to see an example.</summary>

> #### Launch a throughput test with payload size 8 bytes through loopback
>
>
> **Pub**
> ```
> HOST_IP=127.0.0.1 \
> IFACE=lo \
> CYCLONEDDS_URI=file://$(pwd)/cyclonedds.xml \
>     ./throughput/target/cdds_pub -p PAYLOAD_SIZE
> ```
>
> **Sub**
> ```
> HOST_IP=127.0.0.1 \
> IFACE=lo \
> CYCLONEDDS_URI=file://$(pwd)/cyclonedds.xml \
>     ./throughput/target/cdds_sub
> ```
>
> Then we get
>
> ```
> 8,1780543.30
> 8,1799334.24
> 8,1635964.43
> 8,1553352.83
> 8,2108886.13
> ```

</details>

### Fine tuning

We finetune the throughput test to achieve optimal performance under our 100GbE netowrk.
The details can be found in [this config](./cyclonedds-fine-tuned-for-throughput.xml).


## Run Latency Test

**Pong**
```
HOST_IP=PONG_IP \
IFACE=PONG_IFACE \
CYCLONEDDS_URI=file://$(pwd)/cyclonedds.xml \
    ./latency/target/cdds_pong
```

**Ping**
```
HOST_IP=PING_IP \
IFACE=PING_IFACE \
CYCLONEDDS_URI=file://$(pwd)/cyclonedds.xml \
    ./latency/target/cdds_ping -i INTERVAL_IN_SECONDS -p PAYLOAD_SIZE_IN_BYTES
```

The results is formatted as

```bash
INTERVAL_IN_SECONDS, LATENCY_IN_MICRO_SECONDS
```

<details>
<summary>Click me to see an example.</summary>

> #### Launch a latency test with payload size 64 bytes and interval 0.1 second through loopback
>
> **Pong**
> ```
> HOST_IP=127.0.0.1 \
> IFACE=lo \
> CYCLONEDDS_URI=file://$(pwd)/cyclonedds.xml \
>     ./latency/target/cdds_pong
> ```
>
> **Ping**
> ```
> HOST_IP=127.0.0.1 \
> IFACE=lo \
> CYCLONEDDS_URI=file://$(pwd)/cyclonedds.xml \
>     ./latency/target/cdds_ping -i 0.1 -p 64
> ```
>
> Then we get
>
> ```
> 0.1000000000,47
> 0.1000000000,44
> 0.1000000000,44
> 0.1000000000,43
> 0.1000000000,44
> ```

</details>
