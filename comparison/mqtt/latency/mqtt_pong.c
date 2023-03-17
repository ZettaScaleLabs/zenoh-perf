//
// Copyright (c) 2017, 2020 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ADLINK zenoh team, <zenoh@adlink-labs.tech>
//
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "MQTTAsync.h"
#include <assert.h>
#include <pthread.h>
#include <unistd.h>
#include <ctype.h>
#include <sys/time.h>


#define CLIENTID    "mqtt_pong"
#define PING_TOPIC  "/test/ping"
#define PONG_TOPIC  "/test/pong"
#define QOS         0

volatile int finished = 0;
volatile int ready = 0;
volatile int subscribed = 0;
u_int64_t counter = 0;
const char* DEFAULT_BROKER = "tcp://127.0.0.1:1883";
const char* DEFAULT_PING_TOPIC = "/test/ping";


int msgarrvd(void *context, char *topicName, int topicLen, MQTTAsync_message *message)
{
    MQTTAsync client = (MQTTAsync)context;
    int rc = -1;
    if (strncmp(topicName, DEFAULT_PING_TOPIC, topicLen) == 0 ) {
        //ping message arrived, sending back
        while (rc != MQTTASYNC_SUCCESS) {
            if ((rc = MQTTAsync_sendMessage(client, PONG_TOPIC, message, NULL)) != MQTTASYNC_SUCCESS) {
                printf("Error sending message: %d\n", rc);
                fflush(stdout);
            }
        }

    }
    MQTTAsync_freeMessage(&message);
    MQTTAsync_free(topicName);
    return 1;
}


void onSubscribe(void* context, MQTTAsync_successData5* response)
{
    subscribed = 1;
}

void onSubscribeFailure(void* context, MQTTAsync_failureData5* response)
{
    printf("Subscribe failed, rc %d\n", response->code);
    exit(EXIT_FAILURE);
}


void onConnectFailure(void* context, MQTTAsync_failureData5* response)
{
    printf("Connect failed, rc %d\n", response->code);
    exit(EXIT_FAILURE);
}


void onConnect(void* context, MQTTAsync_successData5* response)
{
    ready = 1;
}


int main(int argc, char* argv[])
{
    MQTTAsync client;
    MQTTAsync_connectOptions conn_opts = MQTTAsync_connectOptions_initializer5;
    MQTTAsync_createOptions create_opts = MQTTAsync_createOptions_initializer;
    //struct timespec start, end;
    int rc, c;
    char* broker = NULL;

     // Parsing arguments
    while((c = getopt(argc, argv, ":b:")) != -1 ){
        switch (c) {
            case 'b':
                broker = optarg;
                break;
            default:
                break;
        }
    }

    // Setting defaults

    if (broker == NULL) {
        // We should copy the default
        broker = (char*) calloc(sizeof(char),strlen(DEFAULT_BROKER));
        memcpy(broker, DEFAULT_BROKER, strlen(DEFAULT_BROKER));
    }



    create_opts.MQTTVersion = MQTTVERSION_5;
    if ((rc = MQTTAsync_createWithOptions(&client, broker, CLIENTID, MQTTCLIENT_PERSISTENCE_NONE, NULL, &create_opts)) != MQTTASYNC_SUCCESS)
    {
        printf("Failed to create client object, return code %d\n", rc);
        exit(EXIT_FAILURE);
    }



    conn_opts.keepAliveInterval = 3;
    conn_opts.onSuccess5 = onConnect;
    conn_opts.onFailure5 = onConnectFailure;
    conn_opts.context = client;
    conn_opts.MQTTVersion = MQTTVERSION_5;
    conn_opts.cleanstart = 1;
    if ((rc = MQTTAsync_connect(client, &conn_opts)) != MQTTASYNC_SUCCESS)
    {
        printf("Failed to start connect, return code %d\n", rc);
        MQTTAsync_destroy(&client);
        exit(EXIT_FAILURE);
    }

    while (!ready);

    MQTTAsync_responseOptions opts = MQTTAsync_responseOptions_initializer;

    opts.onSuccess5 = onSubscribe;
    opts.onFailure5 = onSubscribeFailure;
    opts.context = client;
    if ((rc = MQTTAsync_subscribe(client, PING_TOPIC, QOS, &opts)) != MQTTASYNC_SUCCESS)
    {
        printf("Failed to start subscribe, return code %d\n", rc);
        finished = 1;
    }

    if ((rc = MQTTAsync_setCallbacks(client, client, NULL, msgarrvd, NULL)) != MQTTASYNC_SUCCESS)
    {
        printf("Failed to set callbacks, return code %d\n", rc);
        MQTTAsync_destroy(&client);
        exit(EXIT_FAILURE);
    }

    while (!subscribed) ;

    while (1) {
            sleep(1);
    }

    exit(EXIT_SUCCESS);
}
