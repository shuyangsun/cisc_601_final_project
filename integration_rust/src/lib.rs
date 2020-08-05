use num_traits::{Float, NumCast};
use std::ops::AddAssign;

pub trait IntegrationFloat: Float + From<f32> + From<u16> + AddAssign<Self> {}

impl IntegrationFloat for f32 {}
impl IntegrationFloat for f64 {}

pub fn trapezoidal<T>(func: fn(T) -> T, n: u16, left: T, right: T) -> T
where
    T: IntegrationFloat,
{
    let mut result: T = (func(left) + func(right)) * NumCast::from(0.5f32).unwrap();
    let delta: T = (right - left) / NumCast::from(n).unwrap();
    let mut cur: T = left + delta;
    while cur < right {
        result += func(cur);
        cur += delta;
    }
    return result * delta;
}

pub fn simpson_1_3<T>(func: fn(T) -> T, n: u16, left: T, right: T) -> T
where
    T: IntegrationFloat,
{
    let mut result = (func(left) + func(right)) * NumCast::from(0.5f32).unwrap();
    let delta: T = (right - left) / NumCast::from(n).unwrap();
    let mut i = 0usize;
    let mut cur: T = left + delta;
    while cur < right {
        result += func(cur)
            * if i % 2 == 0 {
                NumCast::from(4.0f32).unwrap()
            } else {
                NumCast::from(2.0f32).unwrap()
            };
        i += 1;
        cur += delta;
    }
    return delta * result / NumCast::from(3.0f32).unwrap();
}

pub fn simpson_3_8<T>(func: fn(T) -> T, n: u16, left: T, right: T) -> T
where
    T: IntegrationFloat,
{
    let mut result = (func(left) + func(right)) * NumCast::from(0.5f32).unwrap();
    let delta: T = (right - left) / NumCast::from(n).unwrap();
    let mut i = 0usize;
    let mut cur: T = left + delta;
    while cur < right {
        result += func(cur)
            * if (i + 1) % 3 == 0 {
                NumCast::from(2.0f32).unwrap()
            } else {
                NumCast::from(3.0f32).unwrap()
            };
        i += 1;
        cur += delta;
    }
    return delta * result * NumCast::from(3.0f32 / 8.0f32).unwrap();
}

pub fn romberg<T>(func: fn(T) -> T, j: u32, k: u32, left: T, right: T) -> T
where
    T: IntegrationFloat,
{
    if k <= 0 {
        return trapezoidal(func, 2u16.pow(j), left, right);
    }
    let four_pow_k: T = NumCast::from(4u32.pow(k)).unwrap();
    return (four_pow_k * romberg(func, j, k - 1, left, right)
        - romberg(func, j - 1, k - 1, left, right))
        / (four_pow_k - NumCast::from(1.0f32).unwrap());
}
