use integration_rust::IntegrationFloat;

fn original_func<T>(x: T) -> T
where
    T: IntegrationFloat,
{
    x
}

fn main() {
    let res = integration_rust::romberg(original_func, 10, 10, 0.0, 2.0);
    println!("{}", res);
}
