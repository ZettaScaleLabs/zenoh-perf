# Test for Bindings

Supported Bindings
- zenoh-c
- zenoh-cpp
- zenoh-python

## Build

```bash
just build
```

## Run Test

```bash
just test
```

## Analysis

```bash
python3 ./analyze-throughput.py LOG_DIR
python3 ./analyze-latency.py LOG_DIR
```
