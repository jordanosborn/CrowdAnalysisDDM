#!/usr/bin/env python3
# https://pjreddie.com/darknet/yolo/
import subprocess as sp  # nosec
import shutil
import natsort
import re

import cv2
import sys
import os

darknet_commands = ["./darknet", "detect", "./cfg/yolov3.cfg", "./yolov3.weights"]


def save_images(video: str, directory: str):
    video_capture = cv2.VideoCapture(video)
    success, image = video_capture.read()
    count = 0
    while success:
        cv2.imwrite(f"{directory}/{count}.jpg", image)
        success, image = video_capture.read()
        count += 1


# TODO: not the quickest way of doing this
if __name__ == "__main__":
    video = sys.argv[1]
    if os.path.isfile(video):
        extension = os.path.splitext(os.path.basename(video))[-1]
        name = os.path.basename(video).replace(extension, "")
        video = "../" + video
        os.chdir("darknet")
        if os.path.isdir("temp"):
            shutil.rmtree("temp")
        os.mkdir("temp")
        save_images(video, "temp")
        img_files = [filenames for (dirpath, dirnames, filenames) in os.walk("temp")][0]
        img_files = natsort.natsorted(
            [f"./temp/{f}" for f in img_files if f != "0.jpg"],
            alg=natsort.ns.IGNORECASE,
        )
        count = 0
        person_regex = re.compile(r"person: (\d{1,3})%")
        with open("../person_count.csv", "a") as count_file:
            for i, f in enumerate(img_files):
                proc = sp.Popen(darknet_commands + [f], stdout=sp.PIPE)
                in_current_frame = len(person_regex.findall(str(proc.communicate()[0])))
                count += in_current_frame
                count_file.write(f"{name}-{i+1}: {in_current_frame}\n")
                if i % 10 == 0:
                    print(f"{100 * i / len(img_files)}% complete.")
            print(f"Average number of people in each frame ~ {count / len(img_files)}")
            count_file.write(f"{name}-avg: {count / len(img_files)}")
        if os.path.isdir("temp"):
            shutil.rmtree("temp")
        os.chdir("..")
