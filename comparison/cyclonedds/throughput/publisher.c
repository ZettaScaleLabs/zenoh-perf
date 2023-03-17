#include "dds/dds.h"
#include "Throughput.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_SAMPLES 1

static dds_return_t rc;


dds_entity_t prepare_dds(dds_entity_t *writer, const char *partitionName) {

    dds_entity_t participant = dds_create_participant(DDS_DOMAIN_DEFAULT, NULL, NULL);
    if (participant < 0) DDS_FATAL("dds_create_participant: %s\n", dds_strretcode(-participant));

    dds_write_set_batch (true);

    dds_entity_t topic = dds_create_topic(participant, &ThroughputModule_DataType_desc, "Throughput", NULL, NULL);
    if (topic < 0) DDS_FATAL("dds_create_topic: %s\n", dds_strretcode(-topic));

    // publisher QoS
    dds_qos_t *pubQos;
    pubQos = dds_create_qos();
    const char *pubParts[1];
    pubParts[0] = partitionName;
    dds_qset_partition(pubQos, 1, pubParts);
    dds_entity_t publisher = dds_create_publisher(participant, pubQos, NULL);
    if (publisher < 0) DDS_FATAL("dds_create_publisher: %s\n", dds_strretcode(-publisher));
    dds_delete_qos(pubQos);

    // data writer QoS
    dds_qos_t *dwQos;
    dwQos = dds_create_qos();
    dds_qset_reliability(dwQos, DDS_RELIABILITY_RELIABLE, DDS_SECS (10));
    dds_qset_history(dwQos, DDS_HISTORY_KEEP_ALL, 0);
    dds_qset_resource_limits(dwQos, MAX_SAMPLES, DDS_LENGTH_UNLIMITED, DDS_LENGTH_UNLIMITED);
    *writer = dds_create_writer(publisher, topic, dwQos, NULL);
    if (*writer < 0) DDS_FATAL("dds_create_writer: %s\n", dds_strretcode(-*writer));
    dds_delete_qos(dwQos);

    return participant;
}

dds_return_t wait_for_reader(dds_entity_t writer, dds_entity_t participant) {
    dds_entity_t waitset;

    rc = dds_set_status_mask(writer, DDS_PUBLICATION_MATCHED_STATUS);
    if (rc < 0) DDS_FATAL("dds_set_status_mask: %s\n", dds_strretcode(-rc));

    waitset = dds_create_waitset(participant);
    if (waitset < 0) DDS_FATAL("dds_create_waitset: %s\n", dds_strretcode(-waitset));

    rc = dds_waitset_attach(waitset, writer, (dds_attach_t)NULL);
    if (rc < 0) DDS_FATAL("dds_waitset_attach: %s\n", dds_strretcode(-rc));

    rc = dds_waitset_wait(waitset, NULL, 0, DDS_SECS(30));
    if (rc < 0) DDS_FATAL("dds_waitset_wait: %s\n", dds_strretcode(-rc));

    return rc;
}

int main(int argc, char** argv) {
    dds_entity_t writer;
    ThroughputModule_DataType sample;

    // parse arguments
    int c;
    uint32_t payloadSize = 8;
    char* partitionName = "Throughput example";

    while((c = getopt(argc, argv, ":p:n:")) != -1 ) {
        switch (c) {
            case 'p':
                payloadSize = (uint32_t) atoi (optarg);
                break;
            case 'n':
                partitionName = optarg;
                break;
            default:
                break;
        }
    }

    dds_entity_t participant = prepare_dds(&writer, partitionName);

    // wait until have a reader
    if (wait_for_reader(writer, participant) == 0) {
        printf ("=== [Publisher]  Did not discover a reader.\n");
        fflush(stdout);
        rc = dds_delete (participant);
        if (rc < 0) DDS_FATAL("dds_delete: %s\n", dds_strretcode(-rc));
        return EXIT_FAILURE;
    }

    // setup the sample data
    sample.payload._buffer = dds_alloc(payloadSize);
    sample.payload._length = payloadSize;
    sample.payload._release = true;
    for (uint32_t i = 0; i < payloadSize; i++) {
        sample.payload._buffer[i] = 'a';
    }

    // start writting
    while (1) {
        rc = dds_write(writer, &sample);
        if (rc != DDS_RETCODE_TIMEOUT && rc < 0) DDS_FATAL("dds_write: %s\n", dds_strretcode(-rc));
    }

    return EXIT_SUCCESS;
}
