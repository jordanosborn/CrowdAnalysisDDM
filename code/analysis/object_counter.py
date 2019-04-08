#!/usr/bin/env python3
# https://pjreddie.com/darknet/yolo/
import subprocess as sp  # nosec
import shutil
import natsort
import re

import cv2
import sys
import os, sqlite3

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
        conn = sqlite3.connect("crowd.sqlite")
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

        person_regex = re.compile(r"person: (\d{1,3})%")
        with conn:
            conn.execute(
                f"create table count_{name} (frame integer primary key, count float)"
            )
        count = 0
        with conn:
            index, counter = [], []
            for i, f in enumerate(img_files):
                proc = sp.Popen(darknet_commands + [f], stdout=sp.PIPE)
                in_current_frame = len(person_regex.findall(str(proc.communicate()[0])))
                count += in_current_frame
                index.append(i + 1)
                counter.append(float(in_current_frame))
                if i % 10 == 0:
                    print(f"{100 * i / len(img_files)}% complete.")
            conn.executemany(
                f"insert into count_{name} values (?, ?)", zip(index, counter)
            )
            print(f"Average number of people in each frame ~ {count / len(img_files)}")
        if os.path.isdir("temp"):
            shutil.rmtree("temp")
        os.chdir("..")
