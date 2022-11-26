use crate::{Combiner, Node, Texture};

pub enum Operation {
    Add,
    Multiply,
    Subtract,
    Divide,
}

pub struct Math {
    fun: fn(&Texture, &Texture, &mut Texture),
}

impl Math {
    pub fn new(op: Operation) -> Self {
        let fun = match op {
            Operation::Add => add,
            Operation::Multiply => mul,
            Operation::Subtract => sub,
            Operation::Divide => div,
        };

        Self { fun }
    }
}

impl Node for Math {}

impl Combiner for Math {
    fn render(
        &mut self,
        _config: &crate::Config,
        inputs: (&Texture, &Texture),
        target: &mut Texture,
    ) {
        (self.fun)(inputs.0, inputs.1, target);
    }
}

fn add(a: &Texture, b: &Texture, res: &mut Texture) {
    for (pixel, x, y) in res.pixels_mut_absolute() {
        let a = a.at_absolute(x, y);
        let b = b.at_absolute(x, y);

        *pixel = a + b;
    }
}

fn mul(a: &Texture, b: &Texture, res: &mut Texture) {
    for (pixel, x, y) in res.pixels_mut_absolute() {
        let a = a.at_absolute(x, y);
        let b = b.at_absolute(x, y);

        *pixel = a * b;
    }
}

fn sub(a: &Texture, b: &Texture, res: &mut Texture) {
    for (pixel, x, y) in res.pixels_mut_absolute() {
        let a = a.at_absolute(x, y);
        let b = b.at_absolute(x, y);

        *pixel = a - b;
    }
}

fn div(a: &Texture, b: &Texture, res: &mut Texture) {
    for (pixel, x, y) in res.pixels_mut_absolute() {
        let a = a.at_absolute(x, y);
        let b = b.at_absolute(x, y);

        *pixel = a / b;
    }
}
