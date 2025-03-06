use std::ops::RangeTo;

use rand::Rng;

pub const PI: f64 = 3.1415926535897932385;
pub const INF: f64 = f64::INFINITY;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::rng().random::<f64>()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    rand::rng().random_range(min..max)
}
