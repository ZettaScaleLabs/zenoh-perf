SHELL := /bin/bash
CFLAGS := -O2 -Wall -g -pg

build: build_throughput build_latency

build_throughput: mqtt-lib mosquitto
	@echo ">>> Compiling throughput..."
	@mkdir -p ./target
	@source ./scripts/set-vars.sh && \
		gcc $(CFLAGS) throughput/mqtt_pub_thr.c -lpaho-mqtt3a -pthread -o target/mqtt_pub_thr && \
		gcc $(CFLAGS) throughput/mqtt_sub_thr.c -lpaho-mqtt3a  -pthread -o target/mqtt_sub_thr
	@echo ">>> Done."

build_latency: mqtt-lib mosquitto
	@echo ">>> Compiling latency..."
	@source ./scripts/set-vars.sh
	@mkdir -p ./target
	@source ./scripts/set-vars.sh && \
		gcc $(CFLAGS) latency/mqtt_ping.c -lpaho-mqtt3a -pthread -o target/mqtt_ping && \
		gcc $(CFLAGS) latency/mqtt_pong.c -lpaho-mqtt3a -pthread -o target/mqtt_pong
	@echo ">>> Done."

build_overhead:
	@echo ">>> Compiling overhead..."
	@source ./scripts/set-vars.sh && \
		gcc $(CFLAGS) overhead/mqtt_overhead.c -lpaho-mqtt3a -pthread -o target/mqtt_overhead
	@echo ">>> Done."

install_mqtt mqtt-lib:
	@./scripts/install-paho.mqtt.c.sh

install_mosquitto mosquitto:
	@./scripts/install-mosquitto.sh

clean:
	@rm -rvf target
