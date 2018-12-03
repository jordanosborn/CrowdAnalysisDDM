import subprocess as sp
from sys import argv, platform, version_info
from os import chdir


def build():
    sp.call(["cmake", "."])
    sp.call(["make"])


def install():
    pass

def run():
    pass


if __name__ == "__main__":
    if version_info.major != 3: raise RuntimeError("Please run build script using python3.")

    directory = "/".join(__file__.split("/")[:-1])
    chdir(directory)
    if "--build" in argv: build()
    elif "--install" in argv: build(); install()
    elif "--run" in argv: build(); run()

