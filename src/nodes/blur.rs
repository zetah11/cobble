use std::f64::consts::PI;

pub fn inverse_blur_sigma<const W: usize, const H: usize>(sigma: f64) -> [[f64; W]; H] {
    let mut matrix = blur_sigma(sigma);

    for value in matrix.iter_mut().flat_map(|row| row.iter_mut()) {
        *value = 1.0 - *value;
    }

    matrix
}

pub fn blur_sigma<const W: usize, const H: usize>(sigma: f64) -> [[f64; W]; H] {
    let ss2 = 2.0 * sigma * sigma;
    let ss2_pi = ss2 * PI;

    let mut res = [[0.0; W]; H];

    for (y, row) in res.iter_mut().enumerate() {
        for (x, value) in row.iter_mut().enumerate() {
            let vx = x as f64 - (W / 2) as f64;
            let vy = y as f64 - (H / 2) as f64;

            let prod = -(vx * vx + vy * vy) / ss2;
            *value = prod.exp() / ss2_pi;
        }
    }

    res
}
