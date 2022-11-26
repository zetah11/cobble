use rand::{thread_rng, Rng};

use crate::{Config, Generator, Node, Texture};

pub struct Random;

impl Node for Random {}

impl Generator for Random {
    fn render(&mut self, _config: &Config, target: &mut Texture) {
        let mut rng = thread_rng();
        for pixel in target.pixels_mut() {
            pixel.0 = rng.gen();
        }
    }
}
