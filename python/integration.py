from typing import Dict, Tuple
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


class Romberg:
    def __init__(self, func, left: float, right: float) -> None:
        self._func = func
        self._left = left
        self._right = right
        self._cache: Dict[Tuple[int, int], float] = dict()

    def calculate(self, j: int, k: int) -> float:
        if k <= 0:
            if (0, 0) in self._cache:
                return self._cache[(0, 0)]
            self._cache[(0, 0)] = trapezoidal(self._func, 2 ** j, self._left, self._right)
            return self._cache[(0, 0)]
        res = self._cache.get((j, k))
        if res is not None:
            return res
        res = (4 ** k * self.calculate(j, k - 1) - self.calculate(j - 1, k - 1)) / (4 ** k - 1)
        self._cache[(j, k)] = res
        return res


def relative_error(expected: float, actual: float) -> float:
    return abs((expected - actual) / expected)


def original_func(x: float) -> float:
    return x * np.e ** (2 * x)


def integral(x: float) -> float:
    return np.e ** (2 * x) * (2 * x - 1) / 4


if __name__ == '__main__':
    analytical_result = integral(3) - integral(0)

    print('Analytical = {:.2f}'.format(analytical_result))

    romberg_calc = Romberg(original_func, 0, 3)
    res = romberg_calc.calculate(10, 10)
    print('Romberg Integration = {:.2f}, e_a={:.2f}, e_t={:.2f}%'.format(res, abs(analytical_result - res), relative_error(analytical_result, res) * 100))
    print(simpson_1_3(original_func, 1000, 0, 3))
