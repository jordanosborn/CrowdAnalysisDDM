#!/bin/bash

if [ "$1" == "stop-all" ]; then
    echo "stopping"
    docker stop `(docker ps -aq)`
elif [ "$1" == "remove-all" ]; then
    echo "removing"
    docker rm `(docker ps -aq)`
elif [ "$1" == "images-remove-all" ]; then
    echo "deleting"
    docker rmi `(docker images -q)`
fi
