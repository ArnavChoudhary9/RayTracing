const INF: f64 = f64::INFINITY;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const EMPTY: Self = Self {
        min: INF,
        max: -INF,
    };

    pub const UNIVERSE: Self = Self {
        min: -INF,
        max: INF
    };

    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn defalut() -> Self {
        Interval { min: INF, max: -INF }
    }

    pub const fn size(&self) -> f64 {
        self.max - self.min
    }

    pub const fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub const fn surrounds(&self, x: f64) -> bool {
        self.min < x && x <= self.max
    }

    pub const fn clamp(&self, x: f64) -> f64 {
        x.min(self.max).max(self.min)
    }
}
