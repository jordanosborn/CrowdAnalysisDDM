#!/usr/bin/env python3
from multiprocessing import pool

import cv2
import os
import sys
import time


def save(data):
    dirpath, _, files = data
    files = list(sorted(filter(lambda s: s.find('.jpg') != -1, files)))
    name = f"output/{dirpath.split('/')[-1]}.avi"
    if name != ".avi" and len(files) != 0:
        frame = cv2.imread(os.path.join(os.curdir, dirpath, files[0]))
        height, width, _ = frame.shape
        video = cv2.VideoWriter(name, cv2.VideoWriter_fourcc(*"DIVX"), 20.0, (width, height))
        for f in files: video.write(cv2.imread(os.path.join(dirpath, f)))
        cv2.destroyAllWindows()
        video.release()
    return name


def save_pool(data): print(save(data), "- complete!")


if __name__ == "__main__":
    if len(sys.argv) >= 2 and os.path.isdir(sys.argv[1]):
        directories = os.walk(sys.argv[1])
        if not os.path.isdir("output"): os.mkdir("output")
        p = pool.Pool(int(sys.argv[2]) if len(sys.argv) == 3 else 1)
        t0 = time.time()
        p.map(save_pool, directories)
        print(f"Generated {len(directories) - 1} video files in {time.time() - t0}s!")
    else: print("No directory supplied!")
