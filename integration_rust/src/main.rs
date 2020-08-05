use integration_rust::IntegrationFloat;

fn original_func<T>(x: T) -> T
where
    T: IntegrationFloat,
{
    x
}

fn main() {
    let mut romberg_calc = integration_rust::Romberg::from(original_func, 0.0, 2.0);
    let res = romberg_calc.calculate(10, 10);
    println!("{}", res);
}
