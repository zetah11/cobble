use crate::{Combiner, Config, Node, Pixel, Texture};

pub struct Dot3;

impl Node for Dot3 {}

impl Combiner for Dot3 {
    fn render(&mut self, _config: &Config, inputs: (&Texture, &Texture), target: &mut Texture) {
        fn s(v: f64) -> f64 {
            2.0 * v - 1.0
        }

        fn i(v: f64) -> f64 {
            (v + 1.0) * 0.5
        }

        for (pixel, x, y) in target.pixels_mut_absolute() {
            let [ar, ag, ab] = inputs.0.at_absolute(x, y).as_rgb();
            let [br, bg, bb] = inputs.1.at_absolute(x, y).as_rgb();

            let (ar, ag, ab) = (s(ar), s(ag), s(ab));
            let (br, bg, bb) = (s(br), s(bg), s(bb));

            let (r, g, b) = (ar * br, ag * bg, ab * bb);
            *pixel = Pixel::from_rgb(i(r), i(g), i(b));
        }
    }
}
