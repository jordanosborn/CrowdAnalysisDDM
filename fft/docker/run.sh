#!/bin/bash
cd "${0%/*}"
if [ "$1" == "build" ]; then
    docker build -t MastersProject .
elif [ "$1" == "run" ]; then
    docker build -t $2 .
    docker run --name $2 -it MastersProject
elif [ "$1" == "attach" ]; then
    docker start $2
    docker exec -it $2  /bin/bash
fi