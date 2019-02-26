#!/usr/local/bin/python3
import cv2, sys, os
from multiprocessing import pool

def save(data):
    dirpath, _, files = data
    files = list(sorted(filter(lambda s: s.find('.jpg') != -1, files)))
    name = f"output/{dirpath.split('/')[-1]}.mp4"
    if name != ".avi" and len(files) != 0:
        direc = os.path.join(os.curdir, dirpath, files[0])
        frame = cv2.imread(direc)

        height, width, layers = frame.shape
        video = cv2.VideoWriter(name, cv2.VideoWriter_fourcc(*"MJPG"), 24, (width,height))
        for f in files:
            video.write(cv2.imread(f))
        cv2.destroyAllWindows()
        video.release()
    return name





if __name__ == "__main__":
    if len(sys.argv) == 2 and os.path.isdir(sys.argv[1]):
        directories = os.walk(sys.argv[1])
        for d in directories:
            print(save(d) + " - complete!")
    else:
        print("No directory supplied!")
