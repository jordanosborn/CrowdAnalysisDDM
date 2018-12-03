import subprocess as sp
from sys import argv
from os import chdir

def main():
    chdir(__file__[:-len("build.py")])
    sp.call(["cmake", "."])
    sp.call(["make"])


if __name__ == "__main__":
    print(__file__)
    main()
