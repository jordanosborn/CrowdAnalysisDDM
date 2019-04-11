#!/usr/bin/env python3
import sys, os
from moviepy.editor import *


def resize(*args):
    if len(args) == 4:
        with VideoFileClip(args[0], audio=False) as clip:
            (w, h) = clip.size
            cropped_clip = clip.crop(
                width=1024, height=1024, x_center=w / 2, y_center=h / 2
            )
            clip = cropped_clip.resize((int(args[2]), int(args[3])))
            clip.write_videofile(args[1])


if __name__ == "__main__":
    resize(*sys.argv[1:])
