#!/usr/bin/env bash

cmake -B ./zenoh-c/build-main
cmake --build ./zenoh-c/build-main

git apply ./zenoh-c-cmake.patch
git apply ./zenoh-cpp-cmake.patch

cmake -B ./zenoh-c/build-tokio
cmake --build ./zenoh-c/build-tokio
