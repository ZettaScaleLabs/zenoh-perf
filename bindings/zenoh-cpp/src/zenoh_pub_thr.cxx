//
// Copyright (c) 2022 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
#include <stdio.h>
#include <string.h>
#include <vector>

#include "zenoh.hxx"
using namespace zenoh;

#define DEFAULT_PKT_SIZE 8

struct args_t {
    unsigned char help_requested;        // -h
    unsigned int size;                   // -s
    char* config_path;                   // -c
};
struct args_t parse_args(int argc, char** argv);

int _main(int argc, char **argv) {
    const char *keyexpr = "test/thr";

    auto args = parse_args(argc, argv);

    if (args.help_requested) {
        std::cout << "\
        -s (optional, int, default=" << DEFAULT_PKT_SIZE << "): the size of the put message in bytes\n\
        -c (optional, string, disabled when backed by pico): the path to a configuration file for the session. If this option isn't passed, the default configuration will be used.\n\
		";
        return 1;
    }

    size_t len = args.size;
    std::vector<char> payload(len, 1);

    Config config;
#ifdef ZENOHCXX_ZENOHC
    if (args.config_path) {
        config = expect<Config>(config_from_file(args.config_path));
    }
#endif
    auto session = expect<Session>(open(std::move(config)));

    PublisherOptions options;
    options.set_congestion_control(Z_CONGESTION_CONTROL_BLOCK);

    auto pub = expect<Publisher>(session.declare_publisher(keyexpr, options));

    while (1) pub.put(payload);
}

char* getopt(int argc, char** argv, char option) {
    for (int i = 0; i < argc; i++) {
        size_t len = strlen(argv[i]);
        if (len >= 2 && argv[i][0] == '-' && argv[i][1] == option) {
            if (len > 2 && argv[i][2] == '=') {
                return argv[i] + 3;
            } else if (i + 1 < argc) {
                return argv[i + 1];
            }
        }
    }
    return NULL;
}

struct args_t parse_args(int argc, char** argv) {
    for (int i = 0; i < argc; i++) {
        if (strcmp(argv[i], "-h") == 0) {
            struct args_t args;
            args.help_requested = 1;
            return args;
        }
    }
    char* arg = getopt(argc, argv, 's');
    unsigned int size = DEFAULT_PKT_SIZE;
    if (arg) {
        size = atoi(arg);
    }
    struct args_t args;
    args.help_requested = 0;
    args.size = size;
    args.config_path = getopt(argc, argv, 'c');
    return args;
}

int main(int argc, char **argv) {
    try {
        _main(argc, argv);
    } catch (ErrorMessage e) {
        std::cout << "Received an error :" << e.as_string_view() << "\n";
    }
}
