use crate::{Combiner, Config, Node, Texture};

pub struct Mix {
    factor: f64,
}

impl Mix {
    pub fn new(factor: f64) -> Self {
        Self { factor }
    }
}

impl Node for Mix {}

impl Combiner for Mix {
    fn render(&mut self, _config: &Config, inputs: (&Texture, &Texture), target: &mut Texture) {
        let t = self.factor;
        let i = 1.0 - t;

        for (pixel, x, y) in target.pixels_mut_absolute() {
            let a = inputs.0.at_absolute(x, y);
            let b = inputs.1.at_absolute(x, y);

            *pixel = i * a + t * b;
        }
    }
}
