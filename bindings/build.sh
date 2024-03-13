#!/usr/bin/env bash

rm -rf ./zenoh-c/build
rm -rf ./zenoh-cpp/build
cmake ./zenoh-c -B ./zenoh-c/build
cmake --build ./zenoh-c/build
cmake ./zenoh-cpp -B ./zenoh-cpp/build
cmake --build ./zenoh-cpp/build
