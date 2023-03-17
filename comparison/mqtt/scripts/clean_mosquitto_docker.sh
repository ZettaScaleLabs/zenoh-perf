#!/usr/bin/env bash
set -e

groups | grep -q docker || {
    echo \"Warning: You may not have permission to execute docker. Try this command:\" >&2
    echo \"         usermod -a -G docker YOUR_USERNAME\" >&2
}

VERSION=${1:-2.0.15}

MQTT_DOCKER="eclipse-mosquitto:$VERSION"
CONTAINER_ID=$(docker ps -a -q --filter "ancestor=$MQTT_DOCKER" --format="{{.ID}}")

if [ ! -z "$CONTAINER_ID" ]; then
    docker rm $(docker stop $CONTAINER_ID) &> /dev/null
fi
