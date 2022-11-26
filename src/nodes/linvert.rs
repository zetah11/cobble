use colorsys::{Hsl, Rgb, RgbRatio};

use crate::{Config, Node, Pixel, Processor, Texture};

pub struct LInvert;

impl Node for LInvert {}

impl Processor for LInvert {
    fn render(&mut self, _config: &Config, input: &Texture, target: &mut Texture) {
        for (pixel, x, y) in target.pixels_mut_absolute() {
            let input = input.at_absolute(x, y);

            let rgb = RgbRatio::from(input.0);
            let rgb = Rgb::from(rgb);

            let mut hsl = Hsl::from(rgb);
            let lightness = hsl.lightness();
            hsl.set_lightness(100.0 - lightness);

            let rgb = Rgb::from(hsl);
            let rgb: [f64; 4] = rgb.as_ratio().into();
            *pixel = Pixel(rgb)
        }
    }
}
