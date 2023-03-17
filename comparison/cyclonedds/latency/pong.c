#include "dds/dds.h"
#include "dds/ddsrt/misc.h"
#include "RoundTrip.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdlib.h>

static dds_entity_t waitSet;
#define MAX_SAMPLES 1


static RoundTripModule_DataType data[MAX_SAMPLES];
static void * samples[MAX_SAMPLES];
static dds_sample_info_t info[MAX_SAMPLES];

static dds_return_t rc;
static dds_entity_t participant;
static dds_entity_t reader;
static dds_entity_t writer;
static dds_entity_t readCond;

void data_available(dds_entity_t rd, void *arg) {
    (void)arg;
    dds_return_t samplesReceived = dds_take(rd, samples, info, MAX_SAMPLES, MAX_SAMPLES);
    if (samplesReceived < 0) DDS_FATAL("dds_take: %s\n", dds_strretcode(-samplesReceived));
    for (int i = 0; i < samplesReceived; i++) {
        if (info[i].valid_data) {
            RoundTripModule_DataType* thisSample = &data[i];
            rc = dds_write_ts(writer, thisSample, info[i].source_timestamp);
            if (rc < 0) DDS_FATAL("dds_write_ts: %d\n", -rc);
        }
    }
}

void prepare_dds(dds_entity_t *wr, dds_entity_t *rd, dds_entity_t *rdcond, dds_listener_t *rdlist) {
    const char *pubPartitions[] = { "pong" };
    const char *subPartitions[] = { "ping" };

    // topic
    dds_entity_t topic = dds_create_topic(
        participant,
        &RoundTripModule_DataType_desc,
        "RoundTrip",
        NULL,
        NULL
    );
    if (topic < 0) DDS_FATAL("dds_create_topic: %s\n", dds_strretcode(-topic));

    // publisher
    dds_qos_t *pubQos;
    pubQos = dds_create_qos();
    dds_qset_partition(pubQos, 1, pubPartitions);
    dds_entity_t publisher = dds_create_publisher(participant, pubQos, NULL);
    if (publisher < 0) DDS_FATAL("dds_create_publisher: %s\n", dds_strretcode(-publisher));
    dds_delete_qos(pubQos);

    // data writer
    dds_qos_t *dwQos;
    dwQos = dds_create_qos();
    dds_qset_reliability(dwQos, DDS_RELIABILITY_RELIABLE, DDS_SECS(10));
    dds_qset_writer_data_lifecycle(dwQos, false);
    *wr = dds_create_writer(publisher, topic, dwQos, NULL);
    if (*wr < 0) DDS_FATAL("dds_create_writer: %s\n", dds_strretcode(-*wr));
    dds_delete_qos(dwQos);

    // subscriber
    dds_qos_t *subQos;
    subQos = dds_create_qos();
    dds_qset_partition(subQos, 1, subPartitions);
    dds_entity_t subscriber = dds_create_subscriber(participant, subQos, NULL);
    if (subscriber < 0) DDS_FATAL("dds_create_subscriber: %s\n", dds_strretcode(-subscriber));
    dds_delete_qos(subQos);

    // data reader
    dds_qos_t *drQos;
    drQos = dds_create_qos();
    dds_qset_reliability(drQos, DDS_RELIABILITY_RELIABLE, DDS_SECS(10));
    *rd = dds_create_reader(subscriber, topic, drQos, rdlist);
    if (*rd < 0) DDS_FATAL("dds_create_reader: %s\n", dds_strretcode(-*rd));
    dds_delete_qos(drQos);

    // attach waitset to reader
    waitSet = dds_create_waitset(participant);
    *rdcond = dds_create_readcondition (*rd, DDS_ANY_STATE);
    rc = dds_waitset_attach (waitSet, *rdcond, *rd);
    if (rc < 0) DDS_FATAL("dds_waitset_attach: %s\n", dds_strretcode(-rc));
    rc = dds_waitset_attach (waitSet, waitSet, waitSet);
    if (rc < 0) DDS_FATAL("dds_waitset_attach: %s\n", dds_strretcode(-rc));
}


int main (int argc, char *argv[]) {
    dds_duration_t waitTimeout = DDS_INFINITY;
    unsigned int i;
    int status;
    dds_attach_t wsresults[1];
    size_t wsresultsize = 1U;

    bool use_listener = false;

    // initialize data
    memset (data, 0, sizeof (data));
    for (i = 0; i < MAX_SAMPLES; i++) {
        samples[i] = &data[i];
    }

    participant = dds_create_participant(DDS_DOMAIN_DEFAULT, NULL, NULL);
    if (participant < 0) DDS_FATAL("dds_create_participant: %s\n", dds_strretcode(-participant));

    dds_listener_t *listener;
    listener = dds_create_listener(NULL);
    dds_lset_data_available(listener, data_available);

    prepare_dds(&writer, &reader, &readCond, listener);
    while (!dds_triggered (waitSet)) {
        status = dds_waitset_wait (waitSet, wsresults, wsresultsize, DDS_INFINITY);
        if (status < 0) DDS_FATAL("dds_waitset_wait: %s\n", dds_strretcode(-status));
    }

    return EXIT_SUCCESS;
}
