import subprocess as sp
from os import chdir
from sys import argv, platform, version_info


def build():
    sp.call(["cmake", "."])
    sp.call(["make"])


def install():
    build()
    if platform == "darwin":
        pass
    elif platform == "unix":
        pass
    else:
        pass


def run():
    build()
    sp.call(["./Code"])


def test():
    pass


def bench(): pass


if __name__ == "__main__":
    if version_info.major != 3 or version_info.minor < 6:
        raise RuntimeError("Please run script using python3.6 or greater")

    directory = "/".join(__file__.split("/")[:-1])
    chdir(directory)
    dispatch = {
        "--build": build,
        "--install": install,
        "--test": test,
        "--bench": bench,
        "--run": run
    }

    if len(argv) == 2 and argv[1] in dispatch.keys():
        dispatch[argv[1]]
    else:
        print("Supplied arguments are invalid ")
