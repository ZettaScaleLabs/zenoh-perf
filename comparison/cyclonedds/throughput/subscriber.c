#include "dds/dds.h"
#include "Throughput.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <signal.h>
#include <assert.h>

#define MAX_SAMPLES 1

static unsigned long long numSamples = 0;
static unsigned long payloadSize = 0;

static ThroughputModule_DataType data[MAX_SAMPLES];
static void* samples[MAX_SAMPLES];
static dds_sample_info_t info[MAX_SAMPLES];


void data_avaiable (dds_entity_t reader, void *arg) {
    (void)arg;

    dds_return_t samplesReceived = dds_take(reader, samples, info, MAX_SAMPLES, MAX_SAMPLES);
    if (samplesReceived < 0) DDS_FATAL("dds_take: %s\n", dds_strretcode(-samplesReceived));

    for (int i = 0; i < samplesReceived; i++) {
        if (info[i].valid_data) {
            ThroughputModule_DataType* thisSample = &data[i];
            payloadSize = thisSample->payload._length;
            numSamples++;
        }
    }
}

dds_entity_t prepare_dds(dds_entity_t *reader, const char *partitionName) {

    dds_entity_t participant = dds_create_participant(DDS_DOMAIN_DEFAULT, NULL, NULL);
    if (participant < 0) DDS_FATAL("dds_create_participant: %s\n", dds_strretcode(-participant));

    // topic
    dds_entity_t topic = dds_create_topic(
        participant,
        &ThroughputModule_DataType_desc,
        "Throughput",
        NULL,
        NULL
    );
    if (topic < 0) DDS_FATAL("dds_create_topic: %s\n", dds_strretcode(-topic));

    // subscriber
    const char *subParts[1];
    subParts[0] = partitionName;
    dds_qos_t *subQos = dds_create_qos();
    dds_qset_partition(subQos, 1, subParts);
    dds_entity_t subscriber = dds_create_subscriber(participant, subQos, NULL);
    if (subscriber < 0) DDS_FATAL("dds_create_subscriber: %s\n", dds_strretcode(-subscriber));
    dds_delete_qos(subQos);

    // listener
    dds_listener_t *rd_listener;
    rd_listener = dds_create_listener(NULL);
    dds_lset_data_available(rd_listener, data_avaiable);

    // data reader
    dds_qos_t *drQos = dds_create_qos();
    dds_qset_reliability(drQos, DDS_RELIABILITY_RELIABLE, DDS_SECS (10));
    dds_qset_history(drQos, DDS_HISTORY_KEEP_ALL, 0);
    dds_qset_resource_limits(drQos, 4000, DDS_LENGTH_UNLIMITED, DDS_LENGTH_UNLIMITED);

    *reader = dds_create_reader(subscriber, topic, drQos, rd_listener);
    if (*reader < 0) DDS_FATAL("dds_create_reader: %s\n", dds_strretcode(-*reader));

    dds_delete_qos(drQos);
    dds_delete_listener(rd_listener);

    memset (data, 0, sizeof (data));
    for (unsigned int i = 0; i < MAX_SAMPLES; i++) {
        samples[i] = &data[i];
    }

    return participant;
}

int main (int argc, char **argv) {
    char* partitionName = "Throughput example";

    // parse arguments
    int c;
    while((c = getopt(argc, argv, ":n:")) != -1 ) {
        switch (c) {
            case 'n':
                partitionName = optarg;
                break;
            default:
                break;
        }
    }

    dds_entity_t reader;
    dds_entity_t participant = prepare_dds(&reader, partitionName);

    while (1) {
        numSamples = 0;
        dds_time_t prevTime = dds_time();
        dds_sleepfor(DDS_SECS(1));
        if (numSamples > 0) {
            double deltaTime = dds_time() - prevTime;
            printf ("%lu,%.2f\n", payloadSize, (double)numSamples / deltaTime * DDS_NSECS_IN_SEC);
            fflush (stdout);
        }
    }

    fflush(stdout);
    (void) dds_set_status_mask(reader, 0);
    return EXIT_SUCCESS;
}
