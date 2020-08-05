#include <iostream>
#include "numerical_integration/lib.hpp"

template<typename T>
T original_func(T const x) {
    return x;
}

int main(int const argc, char const * const * argv) {
    double const res_1 = numerical_integration::trapezoidal(original_func, 100, 0.0, 2.0);
    double const res_2 = numerical_integration::simpson_1_3(original_func, 100, 0.0, 2.0);
    double const res_3 = numerical_integration::simpson_3_8(original_func, 100, 0.0, 2.0);
    numerical_integration::Romberg romberg_calc{ original_func, 0.0, 2.0 };
    double const res_4 = romberg_calc.calculate(3, 3);
    std::cout << res_1 << std::endl;
    std::cout << res_2 << std::endl;
    std::cout << res_3 << std::endl;
    std::cout << res_4 << std::endl;
    return 0;
}
