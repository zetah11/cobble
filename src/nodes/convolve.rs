use crate::{Node, Pixel, Processor, Texture};

pub struct Convolve<const W: usize, const H: usize> {
    by: [[f64; W]; H],
    normalize: bool,
}

impl<const W: usize, const H: usize> Convolve<W, H> {
    pub fn new(by: [[f64; W]; H]) -> Self {
        Self {
            by,
            normalize: true,
        }
    }

    pub fn with_normalize(self, normalize: bool) -> Self {
        Self { normalize, ..self }
    }
}

impl<const W: usize, const H: usize> Node for Convolve<W, H> {}

impl<const W: usize, const H: usize> Processor for Convolve<W, H> {
    fn render(&mut self, _config: &crate::Config, input: &Texture, target: &mut Texture) {
        let weight = if self.normalize {
            1.0 / self
                .by
                .iter()
                .flat_map(|row| row.iter().copied())
                .sum::<f64>()
        } else {
            1.0
        };

        for (pixel, x, y) in target.pixels_mut_absolute() {
            let mut sum = Pixel::ZERO;

            for offy in 0..H {
                for offx in 0..W {
                    let scale = self.by[offy][offx];

                    let offx = offx as isize - (W / 2) as isize;
                    let offy = offy as isize - (H / 2) as isize;

                    let x = x as isize + offx;
                    let y = y as isize + offy;

                    sum += scale * input.at_absolute_wrap(x, y);
                }
            }

            *pixel = sum * weight;
        }
    }
}
