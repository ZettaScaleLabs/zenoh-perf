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


typedef struct {
    volatile unsigned long count;
    z_clock_t start;
    bool sleep;
} z_stats_t;

z_stats_t *z_stats_make() {
    z_stats_t *stats = z_malloc(sizeof(z_stats_t));
    stats->count = 0;
    stats->sleep = false;
    return stats;
}

void on_sample(const z_sample_t *sample, void *context) {
    z_stats_t *stats = (z_stats_t *)context;
    if (stats->sleep) {
        stats->count++;
    }
}
void drop_stats(void *context) {
    const z_stats_t *stats = (z_stats_t *)context;
    z_free(context);
}

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
        -s (required, int): the size of the put message in bytes\n\
        -c (optional, string): the path to a configuration file for the session. If this option isn't passed, the default configuration will be used.\n\
        "
        );
        return 1;
    }

    z_owned_config_t config = args.config_path ? zc_config_from_file(args.config_path) : z_config_default();

    z_owned_session_t s = z_open(z_move(config));
    if (!z_check(s)) {
        printf("Unable to open session!\n");
        exit(-1);
    }

    z_owned_keyexpr_t ke = z_declare_keyexpr(z_loan(s), z_keyexpr("test/thr"));

    z_stats_t *context = z_stats_make();
    z_owned_closure_sample_t callback = z_closure(on_sample, drop_stats, context);
    z_owned_subscriber_t sub = z_declare_subscriber(z_loan(s), z_loan(ke), z_move(callback), NULL);
    if (!z_check(sub)) {
        printf("Unable to create subscriber.\n");
        exit(-1);
    }

    context->count = 0;
    context->sleep = true;
    context->start = z_clock_now();
    while (1) {
        z_sleep_s(1);
        if(context->count > 0){
            context->sleep = false;
            unsigned long elapsed = z_clock_elapsed_ms(&context->start);
            printf("%d,%.3f\n", args.size, (double)(context->count * 1000) / (double)(elapsed));
            fflush(stdout);
            // reset counter and timer
            context->count = 0;
            context->sleep = true;
            context->start = z_clock_now();
        }
    }

    z_undeclare_subscriber(z_move(sub));
    z_undeclare_keyexpr(z_loan(s), z_move(ke));
    z_close(z_move(s));
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
            return (struct args_t){.help_requested = 1};
        }
    }
    char* arg = getopt(argc, argv, 's');
    unsigned int size;
    if (arg) {
        size = atoi(arg);
    }
    else {
        return (struct args_t){.help_requested = 1};
    }
    return (struct args_t){.help_requested = 0,
                           .size = size,
                           .config_path = getopt(argc, argv, 'c')};
}
