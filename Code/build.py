import subprocess as sp
from sys import argv
from os import chdir


def build():
    sp.call(["cmake", "."])
    sp.call(["make"])


def install():
    pass


if __name__ == "__main__":
    directory = "/".join(__file__.split("/")[:-1])
    chdir(directory)
    if "--build" in argv: build()
    if "--install" in argv: install()

