#!/usr/bin/env python3
# https://pjreddie.com/darknet/yolo/
import subprocess

import cv2
import sys
import os

darknet_commands = [
    "./darknet/darknet",
    "detect",
    "./darknet/cfg/yolov3.cfg",
    "./darknet/yolov3.weights",
]


if __name__ == "__main__":
    video = sys.argv[1]
    if os.path.isfile(video):
        extension = os.path.splitext(os.path.basename(video))[-1]
        name = os.path.basename(video).replace(extension, "")
        if os.path.isdir("temp"):
            os.removedirs("temp")
        os.mkdir("temp")
        