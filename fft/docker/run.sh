#!/bin/bash
cd "${0%/*}"
if [ "$1" == "build" ]; then
    docker build -t masters_project .
elif [ "$1" == "run" ]; then
    docker build -t masters_project .
    docker run --name $2 -it masters_project
elif [ "$1" == "attach" ]; then
    docker start $2
    docker exec -it $2  /bin/bash
fi