mod hittable;
mod hittable_list;
mod image_handler;
mod ray;
mod sphere;
mod vec3;
mod interval;
mod camera;
mod utility;

use sphere::Sphere;
use vec3::Vector3;
use camera::Camera;

use std::sync::Arc;

fn main() {
    let mut world = hittable_list::HittableList::new();

    world.add(Arc::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let cam = Camera::new(16.0 / 9.0, 1280, 100, 50);
    cam.render(&world);
}
