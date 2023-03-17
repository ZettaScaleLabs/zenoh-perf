# User Guide

## Build the source

1. Build throughpt and latency programs
    ```bash
    make
    ```

2. Build zenoh router/broker `zenohd`

    Follow the instruction [here](https://github.com/eclipse-zenoh/zenoh#how-to-install-it).

## Run Throughput Test (Peer-to-Peer)

**Pub**

```bash
ZENOH_PERF_DIR/target/release/zenoh_pub_thr \
    --mode peer \
    --listen tcp/PUB_IP:PORT \
    --payload PAYLOAD_SIZE_IN_BYTES
```

**Sub**
```bash
ZENOH_PERF_DIR/target/release/zenoh_sub_thr \
    --mode peer \
    --connect tcp/PUB_IP:PORT \
    --payload PAYLOAD_SIZE_IN_BYTES
```

The results is formatted as

```bash
payload_size_in_bytes, throughput_in_message_per_second
```

<details>
<summary>
Click me to see an example.
</summary>

> #### Launch a P2P throughput test with payload size 8 bytes through loopback
>
> **Pub**
> ```bash
> ZENOH_PERF_DIR/target/release/zenoh_pub_thr \
>     --mode peer \
>     --listen tcp/127.0.0.1:7447 \
>     --payload 8
> ```
>
> **Sub**
> ```bash
> ZENOH_PERF_DIR/target/release/zenoh_sub_thr \
>     --mode peer \
>     --connect tcp/127.0.0.1:7447 \
>     --payload 8
> ```
>
> Then we get
>
> ```txt
> 8,4807103.665
> 8,4711598.903
> 8,4645235.450
> 8,4695050.844
> 8,4575565.990
> ```

</details>

## Run Throughput Test (Brokered)

**Zenohd as Broker**
```bash
zenohd --listen tcp/BROKER_IP:PORT
```

**Pub**
```bash
ZENOH_PERF_DIR/target/release/zenoh_pub_thr \
    --mode client \
    --connect tcp/BROKER_IP:PORT \
    --payload PAYLOAD_SIZE_IN_BYTES
```

**Sub**
```bash
ZENOH_PERF_DIR/target/release/zenoh_sub_thr \
    --mode client \
    --connect tcp/BROKER_IP:PORT \
    --payload PAYLOAD_SIZE_IN_BYTES
```


The results is formatted as

```bash
payload_size_in_bytes, throughput_in_message_per_second
```

<details>

<summary>
Click me to see an example.
</summary>

> #### Launch a brokered throughput test with payload size 8 bytes through loopback
>
> **Zenohd as Broker**
> ```bash
> zenohd --listen tcp/127.0.0.1:7447
> ```
>
> **Pub**
> ```bash
> ZENOH_PERF_DIR/target/release/zenoh_pub_thr \
>     --mode client \
>     --connect tcp/127.0.0.1:7447 \
>     --payload 8
> ```
>
> **Sub**
> ```bash
> ZENOH_PERF_DIR/target/release/zenoh_sub_thr \
>     --mode client \
>     --connect tcp/127.0.0.1:7447 \
>     --payload 8
> ```
>
> Then we get
>
> ```txt
> 8,3497327.064
> 8,3521224.224
> 8,3518260.781
> 8,3523979.948
> 8,3520201.035
> ```

</details>

## Run Latency Test (Peer-to-Peer)

**Pong**
```bash
ZENOH_PERF_DIR/target/release/zenoh_pong \
    --mode peer \
    --listen tcp/PONG_IP:PORT
```

**Ping**
```bash
ZENOH_PERF_DIR/target/release/zenoh_ping \
    --mode peer \
    --connect tcp/PONG_IP:PORT \
    --interval INTERVAL_IN_SECONDS \
    --payload PAYLOAD_SIZE_IN_BYTES
```

The results is formatted as

```bash
INTERVAL_IN_SECONDS, LATENCY_IN_MICRO_SECONDS
```


<details>
<summary>Click me to see an example.</summary>

> #### Launch a P2P latency test with payload size 64 bytes and interval 0.1 second through loopback
>
> **Pong**
> ```bash
> ZENOH_PERF_DIR/target/release/zenoh_pong \
>     --mode peer \
>     --listen tcp/127.0.0.1:7447
> ```
>
> **Ping**
> ```bash
> ZENOH_PERF_DIR/target/release/zenoh_ping \
>     --mode peer \
>     --connect tcp/127.0.0.1:7447 \
>     --interval 0.1 \
>     --payload 64
> ```
>
> Then we get
> ```txt
> 0.1,77
> 0.1,105
> 0.1,101
> 0.1,58
> 0.1,78
> ```

</details>

## Run Latency Test (Brokered)

**Zenohd as Broker**
```bash
zenohd --listen tcp/BROKER_IP:PORT
```

**Pong**
```bash
ZENOH_PERF_DIR/target/release/zenoh_pong \
    --mode client \
    --connect tcp/BROKER_IP:PORT
```

**Ping**
```bash
ZENOH_PERF_DIR/target/release/zenoh_ping \
    --mode client \
    --connect tcp/BROKER_IP:PORT \
    --interval INTERVAL_IN_SECONDS \
    --payload PAYLOAD_SIZE_IN_BYTES
```

The results is formatted as

```bash
INTERVAL_IN_SECONDS, LATENCY_IN_MICRO_SECONDS
```


<details>
<summary>Click me to see an example.</summary>

> #### Launch a brokered latency test with payload size 64 bytes and interval 0.1 second through loopback
>
> **Zenohd as Broker**
> ```bash
> zenohd --listen tcp/127.0.0.1:7447
> ```
>
> **Pong**
> ```bash
> ZENOH_PERF_DIR/target/release/zenoh_pong \
>     --mode client \
>     --connect tcp/127.0.0.1:7447
> ```
>
> **Ping**
> ```bash
> ZENOH_PERF_DIR/target/release/zenoh_ping \
>     --mode client \
>     --connect tcp/127.0.0.1:7447 \
>     --interval 0.1 \
>     --payload 64
> ```
>
> Then we get
> ```txt
> 0.1,166
> 0.1,163
> 0.1,162
> 0.1,184
> 0.1,197
> ```

</details>
