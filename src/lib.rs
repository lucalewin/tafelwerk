pub mod algorithms;
pub mod array;
pub mod ndarray;

/// inludes endpoint
pub fn linear_space(start: f64, end: f64, num: usize) -> Vec<f64> {
    if num == 1 {
        return vec![end];
    }

    let step = (end - start) / (num - 1) as f64;

    (0..num).map(|i| start + step * i as f64).collect()
}

// pub fn log_space(start: f64, stop: f64, num: usize) -> Vec<f64> {
//     todo!()
// }
