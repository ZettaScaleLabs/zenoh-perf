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
#include <chrono>
#include <thread>

#include "zenoh.hxx"
using namespace zenoh;


struct Stats {
    volatile unsigned long count = 0;
    std::chrono::steady_clock::time_point start = {};
    volatile bool sleep = false;

    void operator()(const Sample &) {
        if(sleep) {
            count++;
        }
    }

    void operator()() { /* do nothing */ }
};

struct args_t {
    unsigned char help_requested;        // -h
    unsigned int size;                   // -s
    char* config_path;                   // -c
};
struct args_t parse_args(int argc, char** argv);

int _main(int argc, char **argv) {
    auto args = parse_args(argc, argv);

    if (args.help_requested) {
        std::cout << "\
        -s (required, int): the size of the put message in bytes\n\
        -c (optional, string, disabled when backed by pico): the path to a configuration file for the session. If this option isn't passed, the default configuration will be used.\n\
		";
        return 1;
    }

    Config config;
#ifdef ZENOHCXX_ZENOHC
    if (args.config_path) {
        config = expect(config_from_file(args.config_path));
    }
#endif
    auto session = expect<Session>(open(std::move(config)));

    KeyExpr keyexpr = session.declare_keyexpr("test/thr");

    Stats stats;
    auto subscriber = expect<Subscriber>(session.declare_subscriber(keyexpr, {stats, stats}));

    stats.count = 0;
    stats.sleep = true;
    stats.start = std::chrono::steady_clock::now();
    while (1) {
        std::this_thread::sleep_for(std::chrono::milliseconds((unsigned long)(1000)));
        if(stats.count > 0) {
            stats.sleep = false;
            auto elapsed_ms =
                    std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::steady_clock::now() - stats.start).count();
            // format and print to stdout
            char buff[100];
            snprintf(buff, sizeof(buff), "%d,%.3f\n", args.size, static_cast<double>(stats.count) * 1000.0 / static_cast<double>(elapsed_ms));
            std::string buffStr = buff;
            std::cout << buffStr;
            // reset counter and timer
            stats.count = 0;
            stats.sleep = true;
            stats.start = std::chrono::steady_clock::now();
        }
    }
    subscriber.drop();

    session.undeclare_keyexpr(std::move(keyexpr));

    return 0;
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
    unsigned int size;
    if (arg) {
        size = atoi(arg);
    }
    else {
        struct args_t args;
        args.help_requested = 1;
        return args;
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
