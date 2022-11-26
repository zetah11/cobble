#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Metric {
    Euclidian,
    EuclidianSquared,
    Manhattan,
    Chebyshev,
}

impl Metric {
    /// Get the smallest wrap-around distance between the two points. Assumes the points are in the `[0, 1]` range.
    pub fn min_dist(&self, a: (f64, f64), b: (f64, f64)) -> f64 {
        let (dx, dy) = self.min_deltas(a, b);

        match self {
            Metric::Euclidian => dx.hypot(dy),
            Metric::EuclidianSquared => dx * dx + dy * dy,
            Metric::Manhattan => dx.abs() + dy.abs(),
            Metric::Chebyshev => dx.abs().max(dy.abs()),
        }
    }

    /// Get the smallest difference between the two points in a wrap around square `[0, 1]`.
    fn min_deltas(&self, (ax, ay): (f64, f64), (bx, by): (f64, f64)) -> (f64, f64) {
        let dx = (bx - ax)
            .abs()
            .min((bx - ax + 1.0).abs())
            .min((bx - ax - 1.0).abs());

        let dy = (by - ay)
            .abs()
            .min((by - ay + 1.0).abs())
            .min((by - ay - 1.0).abs());

        (dx, dy)
    }
}
