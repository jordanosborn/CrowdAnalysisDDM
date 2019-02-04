#!/bin/bash
cd "${0%/*}"
if [ "$1" == "stop-all" ]; then
    echo "stopping"
    docker stop `(docker ps -aq)`
elif [ "$1" == "remove-all" ]; then
    echo "removing"
    docker rm `(docker ps -aq)`
elif [ "$1" == "images-remove-all" ]; then
    echo "deleting"
    docker rmi `(docker images -q)`
elif [ "$1" == "build" ]; then
    docker build -t masters_project .
elif [ "$1" == "run" ]; then
    docker build -t masters_project .
    docker run --name $2 -it masters_project
elif [ "$1" == "attach" ]; then
    docker start $2
    docker exec -it $2  /bin/bash
else
    echo "stop-all - stop all running containers"
    echo "remove-all - remove all cotainers"
    echo "images-remove-all - remove all images"
    echo "build - build image"
    echo "run - run image and name it $2"
    echo "attach - attach to named $2 container"
fi