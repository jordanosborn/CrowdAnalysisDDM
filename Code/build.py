import subprocess as sp
from os import chdir
from sys import argv, platform, version_info


def build():
    sp.call(["cmake", "."])
    sp.call(["make"])


def install():
    if platform == "darwin":
        pass
    elif platform == "unix":
        pass
    else:
        pass


def run():
    sp.call(["./Code"])


if __name__ == "__main__":
    if version_info.major != 3 or version_info.minor < 6:
        raise RuntimeError("Please run build script using python3.6 or greater")

    directory = "/".join(__file__.split("/")[:-1])
    chdir(directory)
    if "--build" in argv:
        build()
    elif "--install" in argv:
        build();
        install()
    elif "--run" in argv:
        build();
        run()
