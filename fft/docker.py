#!/usr/bin/python3

import sys
from subprocess import call
if __name__ == "__main__":
    if "stop-all" in sys.argv:
        call(["docker", "stop", "$(docker ps -aq)"])
    if "remove-all" in sys.argv:
        call(["docker", "rm", "$(docker ps -aq)"])
    if "delete-all" in sys.argv:
        call(["docker", "rmi", "$(docker images -q)"])
