use crate::ray::Ray;
use crate::vec3::Vector3;
use crate::interval::Interval;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Vector3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    pub fn new(p: Vector3, normal: Vector3, t: f64, front_face: bool) -> Self {
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }

    pub fn default() -> Self {
        HitRecord {
            p: Vector3::default(),
            normal: Vector3::default(),
            t: 0.0,
            front_face: true,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector3) {
        self.front_face = outward_normal.dot(&r.direction()) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}
