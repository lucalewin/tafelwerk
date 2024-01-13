use std::ops::{Mul, Add};

use crate::linear_space;

/// implements the explicit euler method to numericaly solve the differential equation
pub fn explicit_euler<T>(
    derivative: impl Fn(f64, &T) -> T,
    start: f64,
    end: f64,
    iterations: usize,
    initial: T,
) -> (Vec<f64>, Vec<T>)
where
    f64: Mul<T, Output = T>,
    for <'a> &'a T: Add<T, Output = T>
{
    let t = linear_space(start, end, iterations);
    let mut u = Vec::with_capacity(iterations);

    // set initial values
    u.push(initial);

    // step size
    let h = t[1] - t[0];

    // do one-step explicit euler iterations
    for i in 1..=iterations {
        let prev = u.last().unwrap();
        u.push(prev + h * derivative(t[i-1], prev));
    }

    // return 'x' and 'y' values
    (t, u)
}
