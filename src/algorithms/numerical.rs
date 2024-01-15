use std::ops::{Add, Mul};

use crate::{linear_space, ndarray::NdArray};

/// implements the explicit euler method to numericaly solve the differential equation
pub fn euler_method<T>(
    derivative: impl Fn(f64, &T) -> T,
    start: f64,
    end: f64,
    iterations: usize,
    initial: T,
) -> (Vec<f64>, Vec<T>)
where
    f64: Mul<T, Output = T>,
    for<'a> &'a T: Add<T, Output = T>,
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
        u.push(prev + h * derivative(t[i - 1], prev));
    }

    // return 'x' and 'y' values
    (t, u)
}

pub fn backward_euler_method(
    derivative: fn(f64, f64) -> f64,
    start: f64,
    end: f64,
    initial: f64,
    iterations: usize,
) {
    let t = linear_space(start, end, iterations);
    let mut u = vec![0.0; iterations];

    // set initial value
    u[0] = initial;

    // step size
    let h = t[1] - t[0];
}

pub fn jacobian_matrix() -> Vec<Vec<f64>> {
    todo!()
}
