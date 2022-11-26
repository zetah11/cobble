use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default)]
pub struct Pixel(pub [f64; 4]);

impl Pixel {
    pub const ZERO: Self = Self([0.0, 0.0, 0.0, 0.0]);

    pub fn splat_rgb(v: f64) -> Self {
        Self([v, v, v, 1.0])
    }

    pub fn splat_rgba(v: f64) -> Self {
        Self([v, v, v, v])
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self([r, g, b, 1.0])
    }

    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self([map_u8(r), map_u8(g), map_u8(b), 1.0])
    }

    pub fn from_rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self([r, g, b, a.clamp(0.0, 1.0)])
    }

    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([map_u8(r), map_u8(g), map_u8(b), map_u8(a)])
    }

    pub fn as_rgb(&self) -> [f64; 3] {
        [self.0[0], self.0[1], self.0[2]]
    }

    pub fn as_rgb8_sat(&self) -> [u8; 3] {
        let [r, g, b, ..] = self.0;
        [map_f64_sat(r), map_f64_sat(g), map_f64_sat(b)]
    }

    pub fn as_rgba(&self) -> [f64; 4] {
        self.0
    }

    /// Convert the pixel to an RGBA quadruplet represented by bytes. Clamps the actual pixel values into the `[0, 1]`
    /// range before conversion.
    pub fn as_rgba8_sat(&self) -> [u8; 4] {
        let [r, g, b, a] = self.0;
        [
            map_f64_sat(r),
            map_f64_sat(g),
            map_f64_sat(b),
            map_f64_sat(a),
        ]
    }

    pub fn powf(&self, n: f64) -> Self {
        let [r, g, b, a] = self.0;
        Self([r.powf(n), g.powf(n), b.powf(n), a.powf(n).clamp(0.0, 1.0)])
    }

    pub fn sqrt(&self) -> Self {
        let [r, g, b, a] = self.0;
        Self([r.sqrt(), g.sqrt(), b.sqrt(), a.sqrt().clamp(0.0, 1.0)])
    }
}

impl Add for Pixel {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let [ar, ag, ab, aa] = self.0;
        let [br, bg, bb, ba] = rhs.0;

        Self([ar + br, ag + bg, ab + bb, (aa + ba).clamp(0.0, 1.0)])
    }
}

impl AddAssign for Pixel {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Pixel {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let [ar, ag, ab, aa] = self.0;
        let [br, bg, bb, ba] = rhs.0;

        Self([ar - br, ag - bg, ab - bb, aa.max(ba)])
    }
}

impl SubAssign for Pixel {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Pixel {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let [ar, ag, ab, aa] = self.0;
        let [br, bg, bb, ba] = rhs.0;
        Self([ar * br, ag * bg, ab * bb, (aa * ba).clamp(0.0, 1.0)])
    }
}

impl Mul<f64> for Pixel {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let [r, g, b, a] = self.0;
        Self([r * rhs, g * rhs, b * rhs, a])
    }
}

impl Mul<Pixel> for f64 {
    type Output = Pixel;

    fn mul(self, rhs: Pixel) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Pixel {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div for Pixel {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let [ar, ag, ab, aa] = self.0;
        let [br, bg, bb, ba] = rhs.0;
        Self([ar / br, ag / bg, ab / bb, (aa / ba).clamp(0.0, 1.0)])
    }
}

impl Div<f64> for Pixel {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let [r, g, b, a] = self.0;
        Self([r / rhs, g / rhs, b / rhs, a])
    }
}

impl Div<Pixel> for f64 {
    type Output = Pixel;

    fn div(self, rhs: Pixel) -> Self::Output {
        let [r, g, b, a] = rhs.0;
        Pixel([self / r, self / g, self / b, a])
    }
}

impl DivAssign<f64> for Pixel {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

fn map_u8(byte: u8) -> f64 {
    (byte as f64) / 255.0
}

/// Translate a value clamped to the `[0, 1]` range into a byte `[0, 255]`.
fn map_f64_sat(value: f64) -> u8 {
    let value = value.max(0.0).min(1.0);
    (value * 255.0).round() as u8
}
