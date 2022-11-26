mod pixel;

pub use pixel::Pixel;

pub struct Texture {
    width: usize,
    height: usize,
    buffer: Vec<Pixel>,
}

impl Texture {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![Pixel::ZERO; width * height],
        }
    }

    pub fn at_absolute(&self, x: usize, y: usize) -> Pixel {
        let x = x % self.width;
        let y = y % self.height;

        let ndx = x + y * self.width;
        self.buffer[ndx]
    }

    pub fn at_absolute_wrap(&self, x: isize, y: isize) -> Pixel {
        let x = x.rem_euclid(self.width as isize) as usize;
        let y = y.rem_euclid(self.height as isize) as usize;

        let ndx = x + y * self.width;
        self.buffer[ndx]
    }

    pub fn at_local(&self, x: f64, y: f64) -> Pixel {
        let (x, y) = self.to_absolute(x, y);

        let ndx = x + y * self.width;
        self.buffer[ndx]
    }

    pub fn pixels(&self) -> impl Iterator<Item = Pixel> + '_ {
        self.buffer.iter().copied()
    }

    pub fn pixels_mut(&mut self) -> impl Iterator<Item = &mut Pixel> + '_ {
        self.buffer.iter_mut()
    }

    pub fn pixels_mut_absolute(&mut self) -> impl Iterator<Item = (&mut Pixel, usize, usize)> + '_ {
        let Self { width, height, .. } = *self;

        self.buffer.iter_mut().enumerate().map(move |(ndx, pixel)| {
            let x = ndx % width;
            let y = ndx / height;
            (pixel, x, y)
        })
    }

    pub fn pixels_mut_local(&mut self) -> impl Iterator<Item = (&mut Pixel, f64, f64)> + '_ {
        let Self { width, height, .. } = *self;

        self.buffer.iter_mut().enumerate().map(move |(ndx, pixel)| {
            let x = (ndx % width) as f64 / width as f64;
            let y = (ndx / height) as f64 / height as f64;
            (pixel, x, y)
        })
    }

    pub fn to_absolute(&self, x: f64, y: f64) -> (usize, usize) {
        let w = self.width as f64;
        let h = self.height as f64;

        let x = (x * w).rem_euclid(w) as usize;
        let y = (y * h).rem_euclid(h) as usize;

        (x, y)
    }

    pub fn to_local(&self, x: usize, y: usize) -> (f64, f64) {
        let x = x % self.width;
        let y = y % self.height;

        let x = x as f64 / self.width as f64;
        let y = y as f64 / self.height as f64;
        (x, y)
    }
}
