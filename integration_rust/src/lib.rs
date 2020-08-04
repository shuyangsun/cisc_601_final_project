pub fn trapezoidal(func: fn(f64) -> f64, n: u32, left: f64, right: f64) -> f64 {
    let mut result = (func(left) + func(right)) / 2.0;
    let delta = (right - left) / (n as f64);
    let mut cur = left + delta;
    while cur < right {
        result += func(cur);
        cur += delta;
    }
    return result;
}

pub fn simpson_1_3(func: fn(f64) -> f64, n: u32, left: f64, right: f64) -> f64 {
    let mut result = (func(left) + func(right)) / 2.0;
    let delta = (right - left) / (n as f64);
    let mut i = 0usize;
    let mut cur = left + delta;
    while cur < right {
        result += func(cur) * if i % 2 == 0 { 4.0 } else { 2.0 };
        i += 1;
        cur += delta;
    }
    return delta * result / 3.0;
}

pub fn simpson_3_8(func: fn(f64) -> f64, n: u32, left: f64, right: f64) -> f64 {
    let mut result = (func(left) + func(right)) / 2.0;
    let delta = (right - left) / (n as f64);
    let mut i = 0usize;
    let mut cur = left + delta;
    while cur < right {
        result += func(cur) * if (i + 1) % 3 == 0 { 2.0 } else { 3.0 };
        i += 1;
        cur += delta;
    }
    return delta * result / 3.0;
}

pub fn romberg(func: fn(f64) -> f64, j: u32, k: u32, left: f64, right: f64) -> f64 {
    if k <= 0 {
        return trapezoidal(func, 2u32.pow(j), left, right);
    }
    return (4u32.pow(k) as f64 * romberg(func, j, k - 1, left, right)
        - romberg(func, j - 1, k - 1, left, right))
        / (4u32.pow(k) as f64 - 1.0);
}
