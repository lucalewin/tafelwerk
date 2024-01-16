use std::ops::{Add, Div, Mul, Sub};

use crate::linear_space;

/// implements the explicit euler method to numericaly solve the differential equation
pub fn euler_method<T, U>(
    derivative: impl Fn(&T, &U) -> U,
    start: T,
    end: T,
    iterations: usize,
    initial: U,
) -> (Vec<T>, Vec<U>)
where
    f64: Mul<U, Output = U>,
    for<'a> &'a U: Add<U, Output = U>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<f64, Output = T>,
    T: Div<f64, Output = T>,
    T: Mul<U, Output = U>,
    T: Copy,
{
    let t = linear_space(start, end, iterations);
    let mut u = Vec::with_capacity(iterations);

    // set initial values
    u.push(initial);

    // step size
    let h = t[1] - t[0];

    // do one-step explicit euler iterations
    for i in 1..=iterations {
        let prev_u = u.last().unwrap();
        let prev_t = t.get(i - 1).unwrap();
        u.push(prev_u + h * derivative(prev_t, prev_u));
    }

    // return 'x' and 'y' values
    (t, u)
}

// pub fn backward_euler_method(
//     derivative: fn(f64, f64) -> f64,
//     start: f64,
//     end: f64,
//     initial: f64,
//     iterations: usize,
// ) {
//     let t = linear_space(start, end, iterations);
//     let mut u = vec![0.0; iterations];

//     // set initial value
//     u[0] = initial;

//     // step size
//     let h = t[1] - t[0];
// }

pub fn jacobian_matrix<const N: usize>(
    func: impl Fn([f64; N]) -> [f64; N],
    args: [f64; N],
) -> [[f64; N]; N] {
    let h = 1e-8;
    let mut j = [[0.0; N]; N];

    for i in 0..N {
        let mut s = args;
        s[i] += h;

        let actual = func(args);
        let shifted = func(s);

        for k in 0..N {
            j[k][i] = (shifted[k] - actual[k]) / h;
        }
    }

    j
}
