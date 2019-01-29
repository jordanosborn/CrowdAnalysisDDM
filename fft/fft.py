#!/usr/local/bin/python3
import os
import subprocess as sp
import sys

directory = os.path.dirname(os.path.realpath(__file__))


def build(profile):
    if profile == "":
        sp.call(["cargo", "build", "--package", "fft", "--bin", "fft"])
    else:
        sp.call(["cargo", "build", f"--{profile}", "--package", "fft", "--bin", "fft"])


def test(profile):
    sp.call(["cargo", "test", "--package", "fft", "--bin", "fft"])


def run(profile):
    build(profile)
    if profile == "":
        sp.call([directory + f"/target/debug/fft"])
    else:
        sp.call([directory + f"/target/{profile}/fft"])

def clean(profile):
    sp.call(["cargo", "clean"])

if __name__ == "__main__":
    os.chdir(directory)
    dispatch = {"build": build, "run": run, "test": test, "clean": clean}
    if len(sys.argv) != 2 and sys.argv[1] not in dispatch.keys():
        print("Incorrect arguments given!")
        exit(-1)
    profile = "release" if len(sys.argv) == 3 and sys.argv[2].lower() == "release" else ""
    dispatch_func = dispatch[sys.argv[1]]
    dispatch_func(profile)
