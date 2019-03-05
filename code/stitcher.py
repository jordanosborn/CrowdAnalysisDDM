#!/usr/bin/env python3
import os
import sys
import time
from multiprocessing import pool

import cv2


def save(data):
    dirpath, _, files = data
    files = list(sorted(filter(lambda s: s.find('.jpg') != -1 or s.find('.png') != -1, files)))
    name = f"output/{dirpath.split('/')[-1]}.m4v"
    if name != ".m4v" and len(files) != 0:
        frame = cv2.imread(os.path.join(os.getcwd(), dirpath, files[0]))
        height, width, _ = frame.shape
        video = cv2.VideoWriter(name, cv2.VideoWriter_fourcc(*"H264"), 2.0, (width, height))
        for i, f in enumerate(files):
            img = cv2.imread(os.path.join(dirpath, f))
            cv2.putText(img, f"dt={i+1}", (60, 60), cv2.FONT_HERSHEY_DUPLEX, 2, (255, 255, 255), 2, cv2.LINE_AA)
            video.write(img)
        cv2.destroyAllWindows()
        video.release()
    return name


def save_pool(data): print(save(data), "- complete!")


if __name__ == "__main__":
    if len(sys.argv) >= 2 and os.path.isdir(sys.argv[1]):
        directories = list(os.walk(sys.argv[1]))
        if not os.path.isdir("output"): os.mkdir("output")
        p = pool.Pool(int(sys.argv[2]) if len(sys.argv) == 3 else 1)
        t0 = time.time()
        p.map(save_pool, directories)
        print(f"Generated {len(directories) - 1} video files in {time.time() - t0}s!")
    else: print("No directory supplied!")
