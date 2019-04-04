#!/usr/bin/env python3
import numpy as np
import matplotlib.pyplot as plt
from scipy.optimize import bisect

tau = np.linspace(0.0, 80, 5000)
I = 1.0 - 2.0 * np.exp(-0.2 * tau)


def find_A(B, C, tau, I):
    return np.mean(I) - B * np.mean(np.exp(C * tau))


def find_B(C, tau, I):
    exp_c_tau = np.exp(C * tau)
    return np.cov(exp_c_tau, I)[0][1] / np.var(exp_c_tau)


def find_C(C, tau, I):
    exp_c_tau = np.exp(C * tau)
    tau_exp_c_tau = tau * exp_c_tau
    return np.cov(I, exp_c_tau)[0][1] * np.cov(tau_exp_c_tau, exp_c_tau)[0][1] - np.var(exp_c_tau) * np.cov(I, tau_exp_c_tau)[0][1]


# C = np.linspace(-1.0, 0.4, 5000)
# plt.plot(C, [find_C(c) for c in C])
# plt.show()

C = bisect(find_C, -5, -0.00001, args=(tau, I))
B = find_B(C, tau, I)
A = find_A(B, C, tau, I)
print(f"{A} + {B} * exp({C}t)")

# C = newton(find_root, -0.4)
# B = find_B(C, tau, I)
# A = find_A(B, C, tau, I)

# Ipred = A - B * numpy.exp(C * tau)

# plt.plot(tau, I)
# plt.plot(tau, Ipred)
# plt.show()
