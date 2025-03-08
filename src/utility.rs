use std::ops::RangeTo;
use rand::Rng;

use crate::vec3::Vector3;

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

pub fn random_in_unit_disk() -> Vector3 {
    loop {
        let p = Vector3::new(
            random_double_range(-1.0, 1.0),
            random_double_range(-1.0, 1.0),
            0.0
        );
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}
