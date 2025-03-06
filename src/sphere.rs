use crate::vec3::Vector3;

use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::interval::Interval;

pub struct Sphere {
    center: Vector3,
    radius: f64
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64) -> Self {
        Sphere { center, radius }
    }

    pub fn default() -> Self {
        Sphere { center: Vector3::default(), radius: 0.0 }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().squared_length();
        let h = oc.dot(&r.direction());
        let c = oc.squared_length() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 { return false; }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) { return false; }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        true
    }
}
