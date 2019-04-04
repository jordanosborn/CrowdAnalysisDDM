#!/usr/bin/env python3
from data_clean import data_open
from sys import argv
from matplotlib import pyplot as plt

if __name__ == "__main__":
    index, X, Y = data_open(argv[1])
    plt.plot(X, Y[int(argv[2])])
    plt.show()
