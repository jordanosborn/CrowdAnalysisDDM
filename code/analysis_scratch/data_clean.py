#!/usr/bin/env python3
import sys
from typing import List, Tuple

import matplotlib.pyplot as plt
import numpy as np


def data_open(file: str) -> Tuple[List[float], List[float], List[List[float]]]:
    with open(sys.argv[1]) as f:
        data = f.readlines()
    data = list(map(lambda s: s.replace("\n", "").split(",")[:-1], data))
    X = None
    index = data[0]
    data = data[1:]
    cleaned = []
    for d in data:
        x = []
        y = []
        for v in d:
            s = v.replace("(", "").replace(")", "").split(" ")
            if X is None:
                x.append(float(s[0]))
            y.append(float(s[1]))
        if X is None:
            X = x
        cleaned.append(y)
    return (index, X, cleaned)


if __name__ == "__main__":
    index, x, y = data_open(sys.argv[1])
    plt.plot(x, y[int(sys.argv[2])])
    plt.show()
