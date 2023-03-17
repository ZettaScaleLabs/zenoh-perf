all: install build_cyclonedds build_throughput build_latency

install:
	@git -C cyclonedds checkout 0.10.2rc1 &> /dev/null || git clone https://github.com/eclipse-cyclonedds/cyclonedds -b 0.10.2rc1

build_cyclonedds:
	@cmake -Scyclonedds -Bbuild -DCMAKE_INSTALL_PREFIX=install -DCMAKE_BUILD_TYPE=Release -DENABLE_SHM=NO -DBUILD_DDSPERF=NO
	@cmake --build build --target install --config Release -j

build_throughput:
	@cmake -Sthroughput -Bthroughput/target -DCMAKE_PREFIX_PATH=../install -DCMAKE_BUILD_TYPE=Release
	@cmake --build throughput/target --config Release -j

build_latency:
	@cmake -Slatency -Blatency/target -DCMAKE_PREFIX_PATH=../install -DCMAKE_BUILD_TYPE=Release
	@cmake --build latency/target --config Release -j

clean:
	@rm -rf install build throughput/target latency/target &> /dev/null
