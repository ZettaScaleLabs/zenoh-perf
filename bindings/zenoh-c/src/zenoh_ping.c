#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "zenoh.h"

#define DEFAULT_PKT_SIZE 8
#define PING_TIMEOUT_SEC 1
#define DEFAULT_INTERVAL 0.1

#define handle_error_en(en, msg) \
    do { errno = en; perror(msg); exit(EXIT_FAILURE); } while (0)

z_condvar_t cond;
z_mutex_t mutex;

void callback(const z_sample_t* sample, void* context) { z_condvar_signal(&cond); }
void drop(void* context) { z_condvar_free(&cond); }

struct args_t {
    unsigned int size;             // -s
    char* config_path;             // -c
    float interval;                // -i
    uint8_t help_requested;        // -h
};
struct args_t parse_args(int argc, char** argv);

int main(int argc, char** argv) {
    struct args_t args = parse_args(argc, argv);
    if (args.help_requested) {
        printf(
            "\
        -s (optional, int, default=%d): the size of the payload embedded in the ping and repeated by the pong\n\
        -i (optional, float, default=%f): the interval in seconds between ping messages\n\
        -c (optional, string): the path to a configuration file for the session. If this option isn't passed, the default configuration will be used.\n\
        ",
            DEFAULT_PKT_SIZE, DEFAULT_INTERVAL);
        return 1;
    }
    z_mutex_init(&mutex);
    z_condvar_init(&cond);
    z_owned_config_t config = args.config_path ? zc_config_from_file(args.config_path) : z_config_default();
    z_owned_session_t session = z_open(z_move(config));
    z_keyexpr_t ping = z_keyexpr_unchecked("test/ping");
    z_keyexpr_t pong = z_keyexpr_unchecked("test/pong");
    z_owned_publisher_t pub = z_declare_publisher(z_loan(session), ping, NULL);
    z_owned_closure_sample_t respond = z_closure(callback, drop, (void*)(&pub));
    z_owned_subscriber_t sub = z_declare_subscriber(z_loan(session), pong, z_move(respond), NULL);
    uint8_t* data = z_malloc(args.size);
    for (int i = 0; i < args.size; i++) {
        data[i] = i % 10;
    }
    z_mutex_lock(&mutex);
    while(1) {
        z_sleep_ms(args.interval * 1000);
        z_clock_t measure_start = z_clock_now();
        z_publisher_put(z_loan(pub), data, args.size, NULL);
        int s = z_condvar_wait(&cond, &mutex);
        if (s != 0) {
            handle_error_en(s, "z_condvar_wait");
        }
        unsigned long elapsed = z_clock_elapsed_us(&measure_start);
        printf("%g,%lu\n", args.interval, elapsed/2);
        fflush(stdout);
    }
    z_free(data);
    z_drop(z_move(sub));
    z_drop(z_move(pub));
    z_close(z_move(session));
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
    arg = getopt(argc, argv, 'i');
    float interval = DEFAULT_INTERVAL;
    if (arg) {
        interval = atof(arg);
    }
    return (struct args_t){.help_requested = 0,
                           .size = size,
                           .interval = interval,
                           .config_path = getopt(argc, argv, 'c')};
}
