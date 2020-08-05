#include <iostream>
#include <benchmark/benchmark.h>
#include "numerical_integration/lib.hpp"

#define NUM_SECTIONS ->Arg(1)->Arg(2)->Arg(5)->Arg(10)->Arg(20)->Arg(30)->Arg(40)->Arg(50)->Arg(60)->Arg(70)->Arg(80)->Arg(90)->Arg(100)

template<typename T>
T function(T const x) {
    return x * x * x * 5.0 - 8.0;
}

static void BM_Trapezoidal_f32(benchmark::State& state) {
    for (auto _: state) {
        numerical_integration::trapezoidal(function, state.range(0), float(0.0), float(5.0));
    }
}
BENCHMARK(BM_Trapezoidal_f32)NUM_SECTIONS;

static void BM_Trapezoidal_f64(benchmark::State& state) {
    for (auto _: state) {
        numerical_integration::trapezoidal(function, state.range(0), double(0.0), double(5.0));
    }
}
BENCHMARK(BM_Trapezoidal_f64)NUM_SECTIONS;

static void BM_Simpson13_f32(benchmark::State& state) {
    for (auto _: state) {
        numerical_integration::simpson_1_3(function, state.range(0), float(0.0), float(5.0));
    }
}
BENCHMARK(BM_Simpson13_f32)NUM_SECTIONS;

static void BM_Simpson13_f64(benchmark::State& state) {
    for (auto _: state) {
        numerical_integration::simpson_1_3(function, state.range(0), double(0.0), double(5.0));
    }
}
BENCHMARK(BM_Simpson13_f64)NUM_SECTIONS;

static void BM_Simpson38_f32(benchmark::State& state) {
    for (auto _: state) {
        numerical_integration::simpson_3_8(function, state.range(0), float(0.0), float(5.0));
    }
}
BENCHMARK(BM_Simpson38_f32)NUM_SECTIONS;

static void BM_Simpson38_f64(benchmark::State& state) {
    for (auto _: state) {
        numerical_integration::simpson_3_8(function, state.range(0), double(0.0), double(5.0));
    }
}
BENCHMARK(BM_Simpson38_f64)NUM_SECTIONS;

BENCHMARK_MAIN();
