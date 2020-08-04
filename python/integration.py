import numpy as np

# Generic functions


def trapezoidal(func, n: int, left: float, right: float) -> float:
    res = (func(left) + func(right)) / 2
    delta = (right - left) / n
    for x in np.arange(left + delta, right, delta):
        res += func(x)
    return delta * res


def simpson_1_3(func, n: int, left: float, right: float) -> float:
    res = func(left) + func(right)
    delta = (right - left) / n
    for i, x in enumerate(np.arange(left + delta, right, delta)):
        res += func(x) * (4 if i % 2 == 0 else 2)
    return delta * res / 3


def simpson_3_8(func, n: int, left: float, right: float) -> float:
    res = func(left) + func(right)
    delta = (right - left) / n
    for i, x in enumerate(np.arange(left + delta, right, delta)):
        res += func(x) * (2 if (i + 1) % 3 == 0 else 3)
    return delta * res * 3 / 8


def romberg(func, j: int, k: int, left: float, right: float) -> float:
    if k <= 0:
        return trapezoidal(func, 2 ** j, left, right)
    return (4 ** k * romberg(func, j, k - 1, left, right) - romberg(func, j - 1, k - 1, left, right)) / (4 ** k - 1)


def relative_error(expected: float, actual: float) -> float:
    return abs((expected - actual) / expected)

# Problem 22.1:
# Integrate xe^(2x) from 0 to 3

print('Problem 22.1:')

# a). Analytically

def original_func(x: float) -> float:
    return x * np.e ** (2 * x)


def integral(x: float) -> float:
    return np.e ** (2 * x) * (2 * x - 1) / 4


analytical_result = integral(3) - integral(0)

print('Analytical = {:.2f}'.format(analytical_result))

res = romberg(original_func, 3, 3, 0, 3)
print('Romberg Integration = {:.2f}, e_a={:.2f}, e_t={:.2f}%'.format(res, abs(analytical_result - res), relative_error(analytical_result, res) * 100))
