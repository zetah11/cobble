use std::f64::consts::SQRT_2;

use rand::{thread_rng, Rng};

use crate::util::Metric;
use crate::{Config, Generator, Node, Pixel, Texture};

pub struct Voronoi {
    num_points: usize,
    points: Vec<(f64, f64)>,
    metric: Metric,
}

impl Voronoi {
    pub fn new(num_points: usize) -> Self {
        Self {
            num_points,
            points: vec![],
            metric: Metric::Euclidian,
        }
    }

    pub fn with_metric(self, metric: Metric) -> Self {
        Self { metric, ..self }
    }
}

impl Node for Voronoi {
    fn init(&mut self, _config: &Config) {
        let mut rng = thread_rng();

        for _ in 0..self.num_points {
            self.points.push(rng.gen());
        }
    }
}

impl Generator for Voronoi {
    fn render(&mut self, config: &Config, target: &mut Texture) {
        const MAX_DIST: f64 = SQRT_2;

        let mut dists = vec![0.0; config.width * config.height];
        for (ndx, dist) in dists.iter_mut().enumerate() {
            let x = ndx % config.width;
            let y = ndx / config.height;

            let (x, y) = target.to_local(x, y);

            *dist = MAX_DIST;
            for point in self.points.iter().copied() {
                *dist = dist.min(self.metric.min_dist((x, y), point));
            }
        }

        let max = dists.iter().copied().reduce(|a, b| a.max(b)).unwrap();

        for (pixel, dist) in target.pixels_mut().zip(dists) {
            *pixel = Pixel::splat_rgb(dist / max)
        }
    }
}
