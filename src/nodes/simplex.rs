use std::f64::consts::TAU;

use noise::{Fbm, NoiseFn, OpenSimplex};
use rand::{thread_rng, Rng};

use crate::{Config, Generator, Node, Pixel, Texture};

pub struct Simplex {
    noise: Fbm<OpenSimplex>,
    scale: f64,
}

impl Simplex {
    pub fn new(scale: f64) -> Self {
        let seed = thread_rng().gen_range(0..10000);

        Self {
            noise: Fbm::new(seed),
            scale,
        }
    }
}

impl Default for Simplex {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl Node for Simplex {}

impl Generator for Simplex {
    fn render(&mut self, _config: &Config, target: &mut Texture) {
        fn i(v: f64) -> f64 {
            (v + 1.0) / 2.0
        }

        let s = self.scale;

        for (pixel, x, y) in target.pixels_mut_local() {
            let (x, y) = (TAU * x, TAU * y);

            let (x, y, z, w) = (s * x.cos(), s * y.cos(), s * x.sin(), s * y.sin());
            let pos1 = [x, y, z, w];
            let pos2 = [x + 10.0, y + 10.0, z + 10.0, w + 10.0];
            let pos3 = [x - 10.0, y - 10.0, z - 10.0, w - 10.0];

            let r = i(self.noise.get(pos1));
            let g = i(self.noise.get(pos2));
            let b = i(self.noise.get(pos3));

            *pixel = Pixel::from_rgb(r, g, b);
        }
    }
}
