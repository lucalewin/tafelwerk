use tafelwerk::{ndarray::NdArray, algorithms::explicit_euler};

fn main() {
    let (x, y) = approx_position(0.5, 1.0);
    println!("x = {:?}", x);
    println!("y = {:?}", y);
}

pub fn approx_position(z: f64, a: f64) -> (Vec<f64>, Vec<f64>) {
    const T: f64 = 1.;
    const N: usize = 1024;

    let s0 = 9.;
    let x0 = 0.;
    let y0 = 1.75;

    const P: f64 = 0.45;
    const CW: f64 = 1.21;
    const D: f64 = 0.24;
    const M: f64 = 0.6;
    const G: f64 = 9.81;

    let vx0 = s0 * a.cos();
    let vy0 = s0 * a.sin();

    let initial_values = NdArray::from_array(&[x0, y0, vx0, vy0]);

    const C: f64 = -P * CW * std::f64::consts::FRAC_PI_8 * D * D / M;

    let derivative = move |_, u: &NdArray| -> NdArray {
        let s = (u[2] * u[2] + u[3] * u[3]).sqrt();
        let vx = u[2] * z;
        let vy = u[3] * z;
        let ax = C * s * u[2] * z;
        let ay = C * s * u[3] * z - G;

        NdArray::from_array(&[vx, vy, ax, ay])
    };

    let (_, u) = explicit_euler(derivative, 0.0, T, N, initial_values);

    u.iter().map(|arr| (arr[0], arr[1])).unzip()
}