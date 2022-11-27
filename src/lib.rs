pub mod compiler;
pub mod nodes;
pub mod util;

mod texture;

pub use texture::{Pixel, Texture};
pub use util::Metric;

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub width: usize,
    pub height: usize,
}

impl Config {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

pub trait Node {
    fn init(&mut self, config: &Config) {
        #![allow(unused_variables)]
    }
}

pub trait Generator: Node {
    fn render(&mut self, config: &Config, target: &mut Texture);
}

pub trait Processor: Node {
    fn render(&mut self, config: &Config, input: &Texture, target: &mut Texture);
}

pub trait Combiner: Node {
    fn render(&mut self, config: &Config, inputs: (&Texture, &Texture), target: &mut Texture);
}
