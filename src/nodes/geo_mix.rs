use crate::{Combiner, Config, Node, Texture};

pub struct GeoMix {
    factor: f64,
}

impl GeoMix {
    pub fn new(factor: f64) -> Self {
        Self { factor }
    }
}

impl Node for GeoMix {}

impl Combiner for GeoMix {
    fn render(&mut self, _config: &Config, inputs: (&Texture, &Texture), target: &mut Texture) {
        let t = 2.0 * self.factor;
        let i = 2.0 - t;

        for (pixel, x, y) in target.pixels_mut_absolute() {
            let a = inputs.0.at_absolute(x, y);
            let b = inputs.1.at_absolute(x, y);

            *pixel = (a.powf(i) * b.powf(t)).sqrt();
        }
    }
}
