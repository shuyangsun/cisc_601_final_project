use integration_rust::IntegrationFloat;

fn func_1<T>(x: T) -> T
where
    T: IntegrationFloat,
{
    x
}

fn main() {
    let mut romberg_calc = integration_rust::Romberg::from(func_1, 0.0, 2.0);
    let res = romberg_calc.calculate(10, 100);
    println!("{}", res);
}
