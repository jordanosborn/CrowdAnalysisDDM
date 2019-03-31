#!/usr/bin/env python3
import numpy
import matplotlib.pyplot as plt
from scipy.optimize import newton

tau = numpy.arange(-80, 80, 0.1)
I = 1.0 - 2.0 * numpy.exp(-0.2 * tau)


@numpy.vectorize
def find_A(B, C, tau, I):
    return B * numpy.mean(numpy.exp(C * tau)) - numpy.mean(I)


@numpy.vectorize
def find_B(C, tau, I):
    exp_c_tau = numpy.exp(C * tau)
    return (numpy.mean(I * exp_c_tau) + numpy.mean(I) * numpy.mean(exp_c_tau)) / numpy.var(exp_c_tau)


@numpy.vectorize
def find_root(C):
    exp_c_tau = numpy.exp(C * tau)
    tau_exp_c_tau = tau * exp_c_tau
    mean_I = numpy.mean(I)
    return (mean_I * numpy.mean(exp_c_tau) + numpy.mean(I * exp_c_tau)) * numpy.cov(exp_c_tau, tau_exp_c_tau)[0][1] - numpy.var(exp_c_tau) * (mean_I * numpy.mean(tau_exp_c_tau) + numpy.mean(I * tau_exp_c_tau))


C = numpy.linspace(-3.0, 0.0, 100)
plt.plot(C, find_root(C))
plt.show()


# C = newton(find_root, -0.4)
# B = find_B(C, tau, I)
# A = find_A(B, C, tau, I)

# Ipred = A - B * numpy.exp(C * tau)

# plt.plot(tau, I)
# plt.plot(tau, Ipred)
# plt.show()
