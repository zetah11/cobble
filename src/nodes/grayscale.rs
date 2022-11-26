use lab::Lab;

use crate::{Config, Node, Pixel, Processor, Texture};

pub struct Grayscale;

impl Node for Grayscale {}

impl Processor for Grayscale {
    fn render(&mut self, _config: &Config, input: &Texture, target: &mut Texture) {
        for (pixel, x, y) in target.pixels_mut_absolute() {
            let input = input.at_absolute(x, y).as_rgb8_sat();
            let lightness = Lab::from_rgb(&input).l as f64;
            *pixel = Pixel::splat_rgb(lightness / 100.0);
        }
    }
}
