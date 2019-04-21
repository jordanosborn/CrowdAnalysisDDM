#!/usr/bin/env python3
from numpy import *
from matplotlib import pyplot as plt
import sys

if __name__ == "__main__":
    if len(sys.argv) < 4:
        x1_str = input("X1 = ")  # nosec
        x2_str = input("X2 = ")  # nosec
        y_str = [input("f(x) = ")]  # nosec
    else:
        x1_str = sys.argv[1]
        x2_str = sys.argv[2]
        y_str = sys.argv[3:]
    try:
        x1 = int(x1_str)
    except ValueError:
        x1 = -10 ** 4
    try:
        x2 = int(x2_str)
    except ValueError:
        x2 = 10 ** 4

    x = linspace(x1, x2, 10 ** 6)
    y = [eval(f"lambda x: {s.replace('^', '**')}") for s in y_str]  # nosec
    plt.ylabel(f"f(x)")
    plt.xlabel("x")
    for f, s in zip(y, y_str):
        plt.plot(x, f(x), label=f"f(x) = {s}")
    plt.legend(loc=2, borderaxespad=0.0)
    plt.show()
