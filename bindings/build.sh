#!/usr/bin/env bash


rm -rf ./zenoh-c/build-main
rm -rf ./zenoh-cpp/build-main
cmake ./zenoh-c -B ./zenoh-c/build-main
cmake --build ./zenoh-c/build-main
cmake ./zenoh-cpp -B ./zenoh-cpp/build-main
cmake --build ./zenoh-cpp/build-main

git apply ./zenoh-c-cmake.patch
git apply ./zenoh-cpp-cmake.patch

rm -rf ./zenoh-c/build-tokio
rm -rf ./zenoh-cpp/build-tokio
cmake ./zenoh-c -B ./zenoh-c/build-tokio
cmake --build ./zenoh-c/build-tokio
cmake ./zenoh-cpp -B ./zenoh-cpp/build-tokio
cmake --build ./zenoh-cpp/build-tokio
