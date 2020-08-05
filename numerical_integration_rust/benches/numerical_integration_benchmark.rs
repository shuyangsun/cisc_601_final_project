use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use integration_rust::{simpson_1_3, simpson_3_8, trapezoidal, IntegrationFloat, Romberg};
use num_traits::NumCast;

const LEFT_32: f32 = 0.0f32;
const LEFT_64: f64 = 0.0f64;

const RIGHT_32: f32 = 5.0f32;
const RIGHT_64: f64 = 5.0f64;

const NUM_SECTIONS: [u16; 13] = [1, 2, 5, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
const ROMBERG_J: u32 = 10;

fn func_1<T>(x: T) -> T
where
    T: IntegrationFloat,
{
    x
}

fn func_2<T>(x: T) -> T
where
    T: IntegrationFloat,
{
    x * x
}

fn func_3<T>(x: T) -> T
where
    T: IntegrationFloat,
{
    x * x * x * NumCast::from(5.0f32).unwrap() - NumCast::from(8.0f32).unwrap()
}

fn func_1_f32_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Function 1 f32");

    for num_section in NUM_SECTIONS.iter() {
        group.bench_with_input(
            BenchmarkId::new("Trapezoidal", num_section),
            num_section,
            |b, _| b.iter(|| trapezoidal(func_1, *num_section, LEFT_32, RIGHT_32)),
        );
        group.bench_with_input(
            BenchmarkId::new("Simpson's 1/3", num_section),
            num_section,
            |b, _| b.iter(|| simpson_1_3(func_1, *num_section, LEFT_32, RIGHT_32)),
        );
        group.bench_with_input(
            BenchmarkId::new("Simpson's 3/8", num_section),
            num_section,
            |b, _| b.iter(|| simpson_3_8(func_1, *num_section, LEFT_32, RIGHT_32)),
        );
        let mut romberg_calc = Romberg::from(func_1, LEFT_32, RIGHT_32);
        group.bench_with_input(
            BenchmarkId::new("Romberg", num_section),
            num_section,
            |b, _| b.iter(|| romberg_calc.calculate(ROMBERG_J, *num_section as u32)),
        );
    }
    group.finish();
}

fn func_1_f64_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Function 1 f64");

    for num_section in NUM_SECTIONS.iter() {
        group.bench_with_input(
            BenchmarkId::new("Trapezoidal", num_section),
            num_section,
            |b, _| b.iter(|| trapezoidal(func_1, *num_section, LEFT_64, RIGHT_64)),
        );
        group.bench_with_input(
            BenchmarkId::new("Simpson's 1/3", num_section),
            num_section,
            |b, _| b.iter(|| simpson_1_3(func_1, *num_section, LEFT_64, RIGHT_64)),
        );
        group.bench_with_input(
            BenchmarkId::new("Simpson's 3/8", num_section),
            num_section,
            |b, _| b.iter(|| simpson_3_8(func_1, *num_section, LEFT_64, RIGHT_64)),
        );
        let mut romberg_calc = Romberg::from(func_1, LEFT_64, RIGHT_64);
        group.bench_with_input(
            BenchmarkId::new("Romberg", num_section),
            num_section,
            |b, _| b.iter(|| romberg_calc.calculate(ROMBERG_J, *num_section as u32)),
        );
    }
    group.finish();
}

fn func_2_f32_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Function 2 f32");

    for num_section in NUM_SECTIONS.iter() {
        group.bench_with_input(
            BenchmarkId::new("Trapezoidal", num_section),
            num_section,
            |b, _| b.iter(|| trapezoidal(func_2, *num_section, LEFT_32, RIGHT_32)),
        );
        group.bench_with_input(
            BenchmarkId::new("Simpson's 1/3", num_section),
            num_section,
            |b, _| b.iter(|| simpson_1_3(func_2, *num_section, LEFT_32, RIGHT_32)),
        );
        group.bench_with_input(
            BenchmarkId::new("Simpson's 3/8", num_section),
            num_section,
            |b, _| b.iter(|| simpson_3_8(func_2, *num_section, LEFT_32, RIGHT_32)),
        );
        let mut romberg_calc = Romberg::from(func_2, LEFT_32, RIGHT_32);
        group.bench_with_input(
            BenchmarkId::new("Romberg", num_section),
            num_section,
            |b, _| b.iter(|| romberg_calc.calculate(ROMBERG_J, *num_section as u32)),
        );
    }
    group.finish();
}

fn func_2_f64_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Function 2 f64");

    for num_section in NUM_SECTIONS.iter() {
        group.bench_with_input(
            BenchmarkId::new("Trapezoidal", num_section),
            num_section,
            |b, _| b.iter(|| trapezoidal(func_2, *num_section, LEFT_64, RIGHT_64)),
        );
        group.bench_with_input(
            BenchmarkId::new("Simpson's 1/3", num_section),
            num_section,
            |b, _| b.iter(|| simpson_1_3(func_2, *num_section, LEFT_64, RIGHT_64)),
        );
        group.bench_with_input(
            BenchmarkId::new("Simpson's 3/8", num_section),
            num_section,
            |b, _| b.iter(|| simpson_3_8(func_2, *num_section, LEFT_64, RIGHT_64)),
        );
        let mut romberg_calc = Romberg::from(func_2, LEFT_64, RIGHT_64);
        group.bench_with_input(
            BenchmarkId::new("Romberg", num_section),
            num_section,
            |b, _| b.iter(|| romberg_calc.calculate(ROMBERG_J, *num_section as u32)),
        );
    }
    group.finish();
}

fn func_3_f32_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Function 3 f32");

    for num_section in NUM_SECTIONS.iter() {
        group.bench_with_input(
            BenchmarkId::new("Trapezoidal", num_section),
            num_section,
            |b, _| b.iter(|| trapezoidal(func_3, *num_section, LEFT_32, RIGHT_32)),
        );
        group.bench_with_input(
            BenchmarkId::new("Simpson's 1/3", num_section),
            num_section,
            |b, _| b.iter(|| simpson_1_3(func_3, *num_section, LEFT_32, RIGHT_32)),
        );
        group.bench_with_input(
            BenchmarkId::new("Simpson's 3/8", num_section),
            num_section,
            |b, _| b.iter(|| simpson_3_8(func_3, *num_section, LEFT_32, RIGHT_32)),
        );
        let mut romberg_calc = Romberg::from(func_3, LEFT_32, RIGHT_32);
        group.bench_with_input(
            BenchmarkId::new("Romberg", num_section),
            num_section,
            |b, _| b.iter(|| romberg_calc.calculate(ROMBERG_J, *num_section as u32)),
        );
    }
    group.finish();
}

fn func_3_f64_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Function 3 f64");

    for num_section in NUM_SECTIONS.iter() {
        group.bench_with_input(
            BenchmarkId::new("Trapezoidal", num_section),
            num_section,
            |b, _| b.iter(|| trapezoidal(func_3, *num_section, LEFT_64, RIGHT_64)),
        );
        group.bench_with_input(
            BenchmarkId::new("Simpson's 1/3", num_section),
            num_section,
            |b, _| b.iter(|| simpson_1_3(func_3, *num_section, LEFT_64, RIGHT_64)),
        );
        group.bench_with_input(
            BenchmarkId::new("Simpson's 3/8", num_section),
            num_section,
            |b, _| b.iter(|| simpson_3_8(func_3, *num_section, LEFT_64, RIGHT_64)),
        );
        let mut romberg_calc = Romberg::from(func_3, LEFT_64, RIGHT_64);
        group.bench_with_input(
            BenchmarkId::new("Romberg", num_section),
            num_section,
            |b, _| b.iter(|| romberg_calc.calculate(ROMBERG_J, *num_section as u32)),
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    func_1_f32_benchmark,
    func_1_f64_benchmark,
    func_2_f32_benchmark,
    func_2_f64_benchmark,
    func_3_f32_benchmark,
    func_3_f64_benchmark
);
criterion_main!(benches);
