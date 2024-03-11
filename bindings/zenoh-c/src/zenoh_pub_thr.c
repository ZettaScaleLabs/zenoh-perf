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

#include "zenoh.h"

#define DEFAULT_PKT_SIZE 8

struct args_t {
    unsigned int size;             // -s
    char* config_path;             // -c
    uint8_t help_requested;        // -h
};
struct args_t parse_args(int argc, char** argv);

int main(int argc, char **argv) {
    struct args_t args = parse_args(argc, argv);
    if (args.help_requested) {
        printf(
            "\
        -s (optional, int, default=%d): the size of the put message in bytes\n\
        -c (optional, string): the path to a configuration file for the session. If this option isn't passed, the default configuration will be used.\n\
		",
            DEFAULT_PKT_SIZE);
        return 1;
    }

    char *keyexpr = "test/thr";
    uint8_t *value = (uint8_t *)z_malloc(args.size);
    memset(value, 1, args.size);

    z_owned_config_t config = args.config_path ? zc_config_from_file(args.config_path) : z_config_default();
    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s)) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    z_publisher_options_t options = z_publisher_options_default();
    options.congestion_control = Z_CONGESTION_CONTROL_BLOCK;

    z_owned_publisher_t pub = z_declare_publisher(z_loan(s), z_keyexpr(keyexpr), &options);
    if (!z_check(pub)) {
        printf("Unable to declare publisher for key expression!\n");
        exit(-1);
    }

    while (1) {
        z_publisher_put(z_loan(pub), (const uint8_t *)value, args.size, NULL);
    }

    z_undeclare_publisher(z_move(pub));
    z_close(z_move(s));
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
            return (struct args_t){.help_requested = 1};
        }
    }
    char* arg = getopt(argc, argv, 's');
    unsigned int size = DEFAULT_PKT_SIZE;
    if (arg) {
        size = atoi(arg);
    }
    return (struct args_t){.help_requested = 0,
                           .size = size,
                           .config_path = getopt(argc, argv, 'c')};
}
