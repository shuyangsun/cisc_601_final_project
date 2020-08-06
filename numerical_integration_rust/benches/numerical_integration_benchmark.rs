use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use integration_rust::{simpson_1_3, simpson_3_8, trapezoidal, IntegrationFloat};
use num_traits::NumCast;

const LEFT_64: f64 = 0.0f64;
const RIGHT_64: f64 = 5.0f64;

const NUM_SECTIONS: [u16; 13] = [1, 2, 5, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

fn func<T>(x: T) -> T
where
    T: IntegrationFloat,
{
    x * x * x * NumCast::from(5.0f32).unwrap() - NumCast::from(8.0f32).unwrap()
}

fn f64_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("f64");

    for num_section in NUM_SECTIONS.iter() {
        group.bench_with_input(
            BenchmarkId::new("Trapezoidal", num_section),
            num_section,
            |b, _| b.iter(|| trapezoidal(func, *num_section, LEFT_64, RIGHT_64)),
        );
        group.bench_with_input(
            BenchmarkId::new("Simpson's 1/3", num_section),
            num_section,
            |b, _| b.iter(|| simpson_1_3(func, *num_section, LEFT_64, RIGHT_64)),
        );
        group.bench_with_input(
            BenchmarkId::new("Simpson's 3/8", num_section),
            num_section,
            |b, _| b.iter(|| simpson_3_8(func, *num_section, LEFT_64, RIGHT_64)),
        );
    }
    group.finish();
}

criterion_group!(benches, f64_benchmark);
criterion_main!(benches);
