use crate::{Config, Node, Processor, Texture};

pub struct Brightener {
    factor: f64,
}

impl Brightener {
    pub fn new(factor: f64) -> Self {
        assert!(factor > 0.0, "brightening factor must be positive");
        Self { factor }
    }
}

impl Node for Brightener {}

impl Processor for Brightener {
    fn render(&mut self, _config: &Config, input: &Texture, target: &mut Texture) {
        for (pixel, x, y) in target.pixels_mut_absolute() {
            let input = input.at_absolute(x, y);
            *pixel = input.powf(1.0 / self.factor);
        }
    }
}
