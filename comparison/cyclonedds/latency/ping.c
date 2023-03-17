#include "dds/dds.h"
#include "dds/ddsrt/misc.h"
#include "RoundTrip.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <inttypes.h>

#define MAX_SAMPLES 1


static dds_entity_t waitSet;
static dds_entity_t writer;
static dds_entity_t reader;
static dds_entity_t participant;
static dds_entity_t readCond;

static RoundTripModule_DataType pub_data;
static RoundTripModule_DataType sub_data[MAX_SAMPLES];
static void *samples[MAX_SAMPLES];
static dds_sample_info_t info[MAX_SAMPLES];

static dds_return_t rc;
static double interval = 0;
static uint32_t payloadSize = 64;


dds_entity_t prepare_dds(dds_entity_t *wr, dds_entity_t *rd, dds_entity_t *rdcond) {
    const char *pubPartitions[] = { "ping" };
    const char *subPartitions[] = { "pong" };

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
    dds_qset_reliability(dwQos, DDS_RELIABILITY_RELIABLE, DDS_SECS (10));
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
    *rd = dds_create_reader(subscriber, topic, drQos, NULL);
    if (*rd < 0) DDS_FATAL("dds_create_reader: %s\n", dds_strretcode(-*rd));
    dds_delete_qos(drQos);

    // attach waitset to reader
    waitSet = dds_create_waitset(participant);
    *rdcond = dds_create_readcondition(*rd, DDS_ANY_STATE);
    rc = dds_waitset_attach(waitSet, *rdcond, *rd);
    if (rc < 0) DDS_FATAL("dds_waitset_attach: %s\n", dds_strretcode(-rc));
    rc = dds_waitset_attach(waitSet, waitSet, waitSet);
    if (rc < 0) DDS_FATAL("dds_waitset_attach: %s\n", dds_strretcode(-rc));

    return participant;
}

int main(int argc, char *argv[]) {

    // parse arguments
    int c;
    while ((c = getopt(argc, argv, ":p:i:")) != -1 ) {
        switch (c) {
            case 'p':
                payloadSize = (uint32_t) atol(optarg);
                if (payloadSize > 100 * 1048576) {
                    printf("Payload size too large!\n");
                    fflush(stdout);
                    exit(EXIT_FAILURE);
                }
                break;
            case 'i':
                interval = atof(optarg);
                break;
            default:
                break;
        }
    }

    // initialize data
    memset (&sub_data, 0, sizeof (sub_data));
    memset (&pub_data, 0, sizeof (pub_data));
    for (int i = 0; i < MAX_SAMPLES; i++) {
        samples[i] = &sub_data[i];
    }
    pub_data.payload._length = payloadSize;
    pub_data.payload._buffer = dds_alloc(payloadSize);
    pub_data.payload._release = true;
    pub_data.payload._maximum = 0;
    for (int i = 0; i < payloadSize; i++) {
        pub_data.payload._buffer[i] = 'a';
    }

    participant = dds_create_participant (DDS_DOMAIN_DEFAULT, NULL, NULL);
    if (participant < 0) DDS_FATAL("dds_create_participant: %s\n", dds_strretcode(-participant));

    prepare_dds(&writer, &reader, &readCond);

    dds_time_t sleepInterval = DDS_SECS(interval);

    dds_attach_t wsresults[1];
    size_t wsresultsize = 1U;
    dds_time_t waitTimeout = DDS_SECS(1);

    while (1) {
        dds_sleepfor(sleepInterval);

        // ping
        rc = dds_write_ts(writer, &pub_data, dds_time());
        if (rc < 0) DDS_FATAL("dds_write_ts: %s\n", dds_strretcode(-rc));

        // pong
        rc = dds_waitset_wait(waitSet, wsresults, wsresultsize, waitTimeout);
        if (rc < 0) DDS_FATAL("dds_waitset_wait: %s\n", dds_strretcode(-rc));
        rc = dds_take(reader, samples, info, MAX_SAMPLES, MAX_SAMPLES);
        if (rc < 0) DDS_FATAL("dds_take: %s\n", dds_strretcode(-rc));

        printf("%.10f,%ld\n", interval, (dds_time() - info[0].source_timestamp) / DDS_NSECS_IN_USEC / 2);
        fflush (stdout);
    }

    return EXIT_SUCCESS;
}
