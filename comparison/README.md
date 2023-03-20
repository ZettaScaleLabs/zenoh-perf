# Performance Comparison of Zenoh, MQTT, Kafka, and DDS

## Protocols to Compare

- [Eclipse Zenoh](./zenoh)
- [Eclipse CycloneDDS](./cyclonedds)
- [Eclipse MQTT](./mqtt)
- [Apache Kafka](./kafka)

## Numerical Results

<details>
  <summary>Single-machine Median Throughput (msg/s)</summary>

| Payload size   | CycloneDDS   | Kafka   | MQTT   | Zenoh P2P   | Zenoh brokered   |
|:---------------|:-------------|:--------|:-------|:------------|:-----------------|
| 8 B            | 2.3 M        | 63.5 K  | 34.3 K | 4.3 M       | 3.0 M            |
| 16 B           | 2.5 M        | 63.9 K  | 34.2 K | 4.2 M       | 2.9 M            |
| 32 B           | 2.3 M        | 62.5 K  | 34.1 K | 4.2 M       | 2.9 M            |
| 64 B           | 2.1 M        | 62.5 K  | 35.1 K | 4.1 M       | 2.8 M            |
| 128 B          | 1.9 M        | 61.8 K  | 34.9 K | 3.9 M       | 2.6 M            |
| 256 B          | 1.8 M        | 59.3 K  | 33.9 K | 3.8 M       | 2.4 M            |
| 512 B          | 1.5 M        | 59.6 K  | 34.0 K | 3.5 M       | 2.1 M            |
| 1 KiB          | 1.3 M        | 57.4 K  | 38.0 K | 3.1 M       | 1.7 M            |
| 2 KiB          | 895.5 K      | 56.2 K  | 37.7 K | 2.5 M       | 1.4 M            |
| 4 KiB          | 553.2 K      | 47.5 K  | 38.1 K | 1.8 M       | 844.2 K          |
| 8 KiB          | 246.3 K      | 40.8 K  | 36.3 K | 1.0 M       | 557.7 K          |
| 16 KiB         | 127.2 K      | 32.6 K  | 36.7 K | 449.6 K     | 276.4 K          |
| 32 KiB         | 88.3 K       | 18.6 K  | 33.1 K | 190.6 K     | 131.0 K          |
| 64 KiB         | 48.0 K       | 6.9 K   | 434.5  | 83.5 K      | 64.3 K           |
| 128 KiB        | 22.7 K       | 3.4 K   | 167.0  | 46.8 K      | 37.7 K           |
| 256 KiB        | 12.3 K       | 1.7 K   | 162.0  | 24.3 K      | 19.0 K           |
| 512 KiB        | 5.6 K        | 826.0   | 153.0  | 12.3 K      | 8.6 K            |
| 1 MiB          | 2.1 K        | N/A     | 72.5   | 6.4 K       | 4.0 K            |
| 2 MiB          | 1.2 K        | N/A     | 42.0   | 3.3 K       | 1.9 K            |
| 4 MiB          | 606.0        | N/A     | 23.5   | 1.5 K       | 890.0            |
| 8 MiB          | 294.0        | N/A     | 8.0    | 677.0       | 362.0            |
| 16 MiB         | 131.0        | N/A     | 4.0    | 364.0       | 174.0            |
| 32 MiB         | 34.0         | N/A     | 2.5    | 226.0       | 92.0             |
| 64 MiB         | 17.0         | N/A     | 1.0    | 97.0        | 50.0             |
| 128 MiB        | 11.0         | N/A     | 0.3    | 42.0        | 22.0             |
| 256 MiB        | 6.0          | N/A     | N/A    | 21.0        | 11.0             |
| 512 MiB        | 3.0          | N/A     | N/A    | 13.0        | 6.0              |

</details>

<details>
  <summary>Single-machine Median Throughput (bps)</summary>

| Payload size   | CycloneDDS   | Kafka   | MQTT    | Zenoh P2P   | Zenoh brokered   |
|:---------------|:-------------|:--------|:--------|:------------|:-----------------|
| 8 B            | 147.0 M      | 4.1 M   | 2.2 M   | 274.0 M     | 189.9 M          |
| 16 B           | 318.7 M      | 8.2 M   | 4.4 M   | 537.5 M     | 374.4 M          |
| 32 B           | 597.6 M      | 16.0 M  | 8.7 M   | 1.1 G       | 735.3 M          |
| 64 B           | 1.1 G        | 32.0 M  | 18.0 M  | 2.1 G       | 1.4 G            |
| 128 B          | 1.9 G        | 63.3 M  | 35.7 M  | 4.0 G       | 2.7 G            |
| 256 B          | 3.6 G        | 121.5 M | 69.4 M  | 7.8 G       | 4.9 G            |
| 512 B          | 6.3 G        | 244.2 M | 139.4 M | 14.5 G      | 8.7 G            |
| 1 KiB          | 10.3 G       | 470.5 M | 311.4 M | 25.2 G      | 14.3 G           |
| 2 KiB          | 14.7 G       | 921.4 M | 617.7 M | 40.6 G      | 23.1 G           |
| 4 KiB          | 18.1 G       | 1.6 G   | 1.2 G   | 59.9 G      | 27.7 G           |
| 8 KiB          | 16.1 G       | 2.7 G   | 2.4 G   | 67.1 G      | 36.5 G           |
| 16 KiB         | 16.7 G       | 4.3 G   | 4.8 G   | 58.9 G      | 36.2 G           |
| 32 KiB         | 23.2 G       | 4.9 G   | 8.7 G   | 50.0 G      | 34.4 G           |
| 64 KiB         | 25.2 G       | 3.6 G   | 227.8 M | 43.8 G      | 33.7 G           |
| 128 KiB        | 23.8 G       | 3.5 G   | 175.1 M | 49.1 G      | 39.5 G           |
| 256 KiB        | 25.8 G       | 3.5 G   | 339.7 M | 51.0 G      | 39.8 G           |
| 512 KiB        | 23.6 G       | 3.5 G   | 641.7 M | 51.5 G      | 35.9 G           |
| 1 MiB          | 17.9 G       | N/A     | 608.1 M | 53.7 G      | 33.2 G           |
| 2 MiB          | 20.2 G       | N/A     | 704.6 M | 55.1 G      | 31.4 G           |
| 4 MiB          | 20.3 G       | N/A     | 788.5 M | 49.2 G      | 29.9 G           |
| 8 MiB          | 19.7 G       | N/A     | 536.9 M | 45.4 G      | 24.3 G           |
| 16 MiB         | 17.6 G       | N/A     | 536.9 M | 48.9 G      | 23.4 G           |
| 32 MiB         | 9.1 G        | N/A     | 671.1 M | 60.7 G      | 24.7 G           |
| 64 MiB         | 9.1 G        | N/A     | 536.9 M | 52.1 G      | 26.8 G           |
| 128 MiB        | 11.8 G       | N/A     | 357.6 M | 45.1 G      | 23.6 G           |
| 256 MiB        | 12.9 G       | N/A     | N/A     | 45.1 G      | 23.6 G           |
| 512 MiB        | 12.9 G       | N/A     | N/A     | 55.8 G      | 25.8 G           |
</details>


<details>
  <summary>Muti-machine Median Throughput (msg/s)</summary>

| Payload size   | CycloneDDS   | Kafka   | MQTT   | Zenoh P2P   | Zenoh brokered   |
|:---------------|:-------------|:--------|:-------|:------------|:-----------------|
| 8 B            | 2.1 M        | 67.5 K  | 32.4 K | 4.1 M       | 3.0 M            |
| 16 B           | 2.1 M        | 72.3 K  | 34.5 K | 4.2 M       | 2.9 M            |
| 32 B           | 2.1 M        | 73.7 K  | 33.3 K | 4.0 M       | 2.9 M            |
| 64 B           | 1.8 M        | 72.5 K  | 34.3 K | 3.8 M       | 2.8 M            |
| 128 B          | 1.6 M        | 70.9 K  | 34.4 K | 3.8 M       | 2.5 M            |
| 256 B          | 1.4 M        | 63.7 K  | 35.1 K | 3.5 M       | 2.3 M            |
| 512 B          | 1.1 M        | 63.4 K  | 33.2 K | 3.1 M       | 2.0 M            |
| 1 KiB          | 857.4 K      | 58.6 K  | 34.0 K | 2.5 M       | 1.3 M            |
| 2 KiB          | 581.4 K      | 59.0 K  | 36.5 K | 1.7 M       | 1.1 M            |
| 4 KiB          | 339.2 K      | 49.7 K  | 35.9 K | 1.1 M       | 745.5 K          |
| 8 KiB          | 193.2 K      | 44.3 K  | 34.0 K | 461.2 K     | 399.4 K          |
| 16 KiB         | 102.1 K      | 33.5 K  | 35.4 K | 350.4 K     | 206.3 K          |
| 32 KiB         | 53.2 K       | 18.5 K  | 32.7 K | 153.0 K     | 91.7 K           |
| 64 KiB         | 27.4 K       | 6.2 K   | 318.9  | 79.9 K      | 51.8 K           |
| 128 KiB        | 12.8 K       | 3.2 K   | 73.0   | 42.5 K      | 32.8 K           |
| 256 KiB        | 6.8 K        | 1.7 K   | 10.0   | 22.9 K      | 16.1 K           |
| 512 KiB        | 3.4 K        | 883.0   | 4.5    | 11.6 K      | 7.0 K            |
| 1 MiB          | 1.4 K        | N/A     | 2.5    | 6.0 K       | 3.6 K            |
| 2 MiB          | 750.0        | N/A     | 2.0    | 2.2 K       | 1.9 K            |
| 4 MiB          | 375.0        | N/A     | 1.0    | 925.0       | 919.0            |
| 8 MiB          | 192.0        | N/A     | N/A    | 713.0       | 405.0            |
| 16 MiB         | 93.0         | N/A     | 1.0    | 276.0       | 193.0            |
| 32 MiB         | 27.0         | N/A     | 0.5    | 140.0       | 79.0             |
| 64 MiB         | 14.0         | N/A     | N/A    | 68.0        | 36.0             |
| 128 MiB        | 9.0          | N/A     | N/A    | 36.0        | 18.0             |
| 256 MiB        | 5.0          | N/A     | N/A    | 17.0        | 8.0              |
| 512 MiB        | 2.0          | N/A     | N/A    | 9.0         | 4.0              |

</details>


<details>
  <summary>Muti-machine Median Throughput (bps)</summary>

| Payload size   | CycloneDDS   | Kafka   | MQTT    | Zenoh P2P   | Zenoh brokered   |
|:---------------|:-------------|:--------|:--------|:------------|:-----------------|
| 8 B            | 136.6 M      | 4.3 M   | 2.1 M   | 264.4 M     | 192.3 M          |
| 16 B           | 267.7 M      | 9.3 M   | 4.4 M   | 541.4 M     | 375.0 M          |
| 32 B           | 527.8 M      | 18.9 M  | 8.5 M   | 1.0 G       | 734.0 M          |
| 64 B           | 937.9 M      | 37.1 M  | 17.5 M  | 2.0 G       | 1.4 G            |
| 128 B          | 1.6 G        | 72.6 M  | 35.3 M  | 3.9 G       | 2.6 G            |
| 256 B          | 2.9 G        | 130.5 M | 71.9 M  | 7.2 G       | 4.8 G            |
| 512 B          | 4.6 G        | 259.6 M | 136.0 M | 12.5 G      | 8.3 G            |
| 1 KiB          | 7.0 G        | 479.9 M | 278.8 M | 20.1 G      | 10.9 G           |
| 2 KiB          | 9.5 G        | 967.4 M | 598.1 M | 28.0 G      | 18.0 G           |
| 4 KiB          | 11.1 G       | 1.6 G   | 1.2 G   | 36.9 G      | 24.4 G           |
| 8 KiB          | 12.7 G       | 2.9 G   | 2.2 G   | 30.2 G      | 26.2 G           |
| 16 KiB         | 13.4 G       | 4.4 G   | 4.6 G   | 45.9 G      | 27.0 G           |
| 32 KiB         | 13.9 G       | 4.9 G   | 8.6 G   | 40.1 G      | 24.0 G           |
| 64 KiB         | 14.4 G       | 3.3 G   | 167.2 M | 41.9 G      | 27.1 G           |
| 128 KiB        | 13.4 G       | 3.3 G   | 76.5 M  | 44.5 G      | 34.4 G           |
| 256 KiB        | 14.2 G       | 3.7 G   | 21.0 M  | 48.0 G      | 33.7 G           |
| 512 KiB        | 14.3 G       | 3.7 G   | 18.9 M  | 48.8 G      | 29.6 G           |
| 1 MiB          | 12.0 G       | N/A     | 21.0 M  | 50.7 G      | 29.9 G           |
| 2 MiB          | 12.6 G       | N/A     | 33.6 M  | 37.2 G      | 31.5 G           |
| 4 MiB          | 12.6 G       | N/A     | 33.6 M  | 31.0 G      | 30.8 G           |
| 8 MiB          | 12.9 G       | N/A     | N/A     | 47.8 G      | 27.2 G           |
| 16 MiB         | 12.5 G       | N/A     | 134.2 M | 37.0 G      | 25.9 G           |
| 32 MiB         | 7.2 G        | N/A     | 134.2 M | 37.6 G      | 21.2 G           |
| 64 MiB         | 7.5 G        | N/A     | N/A     | 36.5 G      | 19.3 G           |
| 128 MiB        | 9.7 G        | N/A     | N/A     | 38.7 G      | 19.3 G           |
| 256 MiB        | 10.7 G       | N/A     | N/A     | 36.5 G      | 17.2 G           |
| 512 MiB        | 8.6 G        | N/A     | N/A     | 38.7 G      | 17.2 G           |

</details>


<details>
  <summary>Single-machine Latency (us)</summary>

| Protocol             |   5th Percentile |   Median |   95th Percentile |
|:---------------------|-----------------:|---------:|------------------:|
| Ping                 |              0.5 |      1.0 |               1.0 |
| Zenoh-pico P2P (UDP) |              4.0 |      5.0 |               6.0 |
| CycloneDDS           |              7.0 |      8.0 |               8.0 |
| Zenoh P2P            |             10.0 |     10.0 |              10.0 |
| Zenoh brokered       |             20.0 |     21.0 |              24.0 |
| MQTT                 |             24.0 |     27.0 |              30.0 |
| Kafka                |             60.0 |     70.0 |              87.0 |

</details>

<details>
  <summary>Multi-machine Latency (us)</summary>

| Protocol             |   5th Percentile |   Median |   95th Percentile |
|:---------------------|-----------------:|---------:|------------------:|
| Ping                 |              6.0 |      6.5 |               7.5 |
| Zenoh-pico P2P (UDP) |             11.0 |     12.5 |              14.5 |
| Zenoh P2P            |             15.0 |     16.0 |              17.0 |
| CycloneDDS           |             29.0 |     37.0 |              47.0 |
| Zenoh brokered       |             37.0 |     41.0 |              44.0 |
| MQTT                 |             42.0 |     45.0 |              47.0 |
| Kafka                |             70.0 |     81.0 |             114.0 |

</details>
