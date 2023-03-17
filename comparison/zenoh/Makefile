SHELL := /bin/bash

build: build_throughput build_latency

build_throughput:
	@cargo build --release \
		--bin zenoh_pub_thr \
		--bin zenoh_sub_thr

build_latency:
	@cargo build --release \
		--bin zenoh_ping \
		--bin zenoh_pong

clean:
	@cargo clean
	@rm -f Cargo.lock &> /dev/null
