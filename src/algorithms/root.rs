pub fn bisection(
    func: fn(f64) -> f64,
    mut start: f64,
    mut end: f64,
    tolerance: f64,
    iterations: usize,
) -> f64 {
    if start >= end {
        panic!("start must be smaller than end!") // FIXME
    }

    if !((func(start) < 0.0 && func(end) > 0.0) || (func(start) > 0.0 && func(end) < 0.0)) {
        panic!("conditions not meet"); // FIXME
    }

    for _ in 0..iterations {
        let mid = (start + end) / 2.0;

        if func(mid) == 0.0 || (end - start) / 2.0 < tolerance {
            return mid;
        }

        if func(mid).is_sign_negative() == func(start).is_sign_negative() {
            start = mid;
        } else {
            end = mid;
        }
    }

    panic!("bisection failed!") // FIXME
}

pub fn halleys(
    func: fn(f64) -> f64,
    derivative: fn(f64) -> f64,
    second_derivative: fn(f64) -> f64,
    initial: f64,
    iterations: usize,
) -> f64 {
    let mut x = initial;

    for _ in 0..iterations {
        let z = func(x); // zeroth
        let f = derivative(x); // first
        let s = second_derivative(x); // second
        x -= 2.0 * z * f / (2.0 * f * f - z * s)
    }

    x
}

pub fn newtons(
    func: fn(f64) -> f64,
    derivative: fn(f64) -> f64,
    initial: f64,
    iterations: usize,
) -> f64 {
    let mut x = initial;

    for _ in 0..iterations {
        x -= func(x) / derivative(x)
    }

    x
}

pub fn newtons_ndim

pub fn secant(func: fn(f64) -> f64, start: f64, end: f64, iterations: usize) -> f64 {
    let mut x1 = start;
    let mut x2 = end;

    let mut x = 0.0;

    for _ in 0..iterations {
        let f1 = func(x1);
        let f2 = func(x2);
        let temp = x;
        x = x1 - f1 * (x1 - x2) / (f1 - f2);
        x2 = x1;
        x1 = temp;
    }

    x
}
