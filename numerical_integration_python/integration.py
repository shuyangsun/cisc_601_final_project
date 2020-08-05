import timeit
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


def numeric_function(x: float) -> float:
    return x * x * x * 5 - 8


def run_test(test_name: str, func_name: str, num_section: int) -> None:
    setup_str = 'from integration import {0}, numeric_function'.format(func_name)
    code_str = '{0}(numeric_function, {1}, 0.0, 5.0)'.format(func_name, num_section)
    num_iter = 1000
    duration_ns = timeit.timeit(code_str, setup=setup_str, number=num_iter) / num_iter * 1e+9
    print('{0} {1}: {2:.2f}ns'.format(test_name, num_section, duration_ns))


if __name__ == '__main__':
    num_sections = (1, 2, 5, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100)
    test_dict = {
        'Trapezoidal': 'trapezoidal',
        'Simpson 1/3': 'simpson_1_3',
        'Simpson 3/8': 'simpson_3_8'
    }
    for name, f_name in test_dict.items():
        for n in num_sections:
            run_test(name, f_name, n)
