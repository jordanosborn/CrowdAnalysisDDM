import sys
import subprocess as sp
import os
import multiprocessing


def run(video: str, capacity: int, radial_width: int):
    sp.call(["cargo", "run", "--release", "video",
             capacity, radial_width, video])


if __name__ == "__main__":
    if len(sys.argv) == 4:
        files = []
        capacity, radial_width = int(sys.argv[2]), int(sys.argv[3])
        for (dirpath, dirnames, filenames) in os.walk(sys.argv[1]):
            files.extend(map(lambda s: f"./{dirpath}{s}", filenames))

        for video in files:
            run(video, capacity, radial_width)

    else:
        print(
            f"Arguments supplied are incorrect (_, directory, capacity, radial_width) - {sys.argv}")
