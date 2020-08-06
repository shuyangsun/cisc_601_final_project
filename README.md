# CISC 601 Final Project

This projects implements popular numerical integration methods in three programming languages: Rust, C and Python, then benchmark them in terms of runtime performance.

Numerical integration methods implemented are Trapezoidal rule, Simpson’s 1/3 rule, Simpson’s 3/8 rule, and Romberg. The Rust and C++ version of the code are implemented in a generic way, so both 32-bit and 64-bit floating point calculation can be tested.

In practice, we found that Romberg's method of integration is very efficient even with small number of iterations, and using large number of iteration usually results in numerical overflow, so performance testing is not done for Romberg's method. The final comparison result only includes 64-bit floating point function calls because that is the only floating point number format Python supports natively without third-party libraries.
