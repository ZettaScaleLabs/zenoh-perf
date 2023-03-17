all: build install_kafka

install_kafka:
	@./scripts/install-kafka.sh

build: build_throughput build_latency

build_throughput:
	@cargo build --release --bin kafka_pub_thr --bin kafka_sub_thr

build_latency:
	@cargo build --release --bin kafka_ping --bin kafka_pong

clean:
	@cargo clean
	@rm -f Cargo.lock &> /dev/null
