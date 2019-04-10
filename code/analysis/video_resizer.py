#!/usr/bin/env python3
import sys, os
from moviepy.editor import *


def resize(*args):
    if len(args) == 4:
        with VideoFileClip(args[0], audio=False) as clip:
            clip = clip.resize((int(args[2]), int(args[3])))
            clip.write_videofile(args[1])


if __name__ == "__main__":
    resize(*sys.argv[1:])
