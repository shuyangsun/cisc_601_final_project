use integration_rust::IntegrationFloat;
use num_traits::NumCast;

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

fn main() {
    let mut romberg_calc = integration_rust::Romberg::from(func_1, 0.0, 2.0);
    let res = romberg_calc.calculate(10, 10);
    println!("{}", res);
}
