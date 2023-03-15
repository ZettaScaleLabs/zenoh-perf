# User Guide

## Build the source

```bash
make
```

## Run Throughput Test

**Launch Eclipse Mosquitto as Broker**
```bash
./mosquitto/src/mosquitto -c ./mosquitto.conf
```

**Pub**
```bash
source ./scripts/set-vars.sh
./target/mqtt_pub_thr \
    -p PAYLOAD_SIZE_IN_BYTES \
    -b tcp://BROKER_IP:1883
```

**Sub**
```bash
source ./scripts/set-vars.sh
./target/mqtt_sub_thr \
    -p PAYLOAD_SIZE_IN_BYTES \
    -b tcp://BROKER_IP:1883
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
> **Launch Eclipse Mosquitto as Broker**
> ```bash
> ./mosquitto/src/mosquitto -c ./mosquitto.conf
> ```
>
> **Pub**
> ```bash
> source ./scripts/set-vars.sh
> ./target/mqtt_pub_thr \
>     -p 8 \
>     -b tcp://127.0.0.1:1883
> ```
>
> **Sub**
> ```bash
> source ./scripts/set-vars.sh
> ./target/mqtt_sub_thr \
>     -p 8 \
>     -b tcp://127.0.0.1:1883
> ```
>
> Then we get
>
> ```txt
> 8,27108.510
> 8,30384.086
> 8,30206.430
> 8,30396.389
> 8,30335.090
> ```

</details>

## Run Latency Test

**Launch Eclipse Mosquitto as Broker**
```bash
./mosquitto/src/mosquitto -c ./mosquitto.conf
```

**Pong**
```bash
source ./scripts/set-vars.sh
./target/mqtt_pong \
    -b tcp://BROKER_IP:1883
```

**Ping**
```bash
source ./scripts/set-vars.sh
./target/mqtt_ping \
    -i INTERVAL_IN_SECONDS \
    -p PAYLOAD_SIZE_IN_BYTES \
    -b tcp://BROKER_IP:1883
```

The results is formatted as

```bash
INTERVAL_IN_SECONDS, LATENCY_IN_MICRO_SECONDS
```

<details>
<summary>Click me to see an example.</summary>

> #### Launch a brokered latency test with payload size 64 bytes and interval 0.1 second through loopback
>
> **Launch Eclipse Mosquitto as Broker**
> ```bash
> ./mosquitto/src/mosquitto -c ./mosquitto.conf
> ```
>
> **Pong**
> ```bash
> source ./scripts/set-vars.sh
> ./target/mqtt_pong \
>     -b tcp://127.0.0.1:1883
> ```
>
> **Ping**
> ```bash
> source ./scripts/set-vars.sh
> ./target/mqtt_ping \
>     -i 0.1 \
>     -p 64 \
>     -b tcp://127.0.0.1:1883
> ```
>
> Then we get
> ```txt
> 0.1000000000,191
> 0.1000000000,100
> 0.1000000000,161
> 0.1000000000,89
> 0.1000000000,108
> ```

</details>
