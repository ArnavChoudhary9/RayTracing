use crate::vec3::Vector3;
use crate::image_handler;
use crate::hittable;
use crate::ray::Ray;
use crate::interval::Interval;

use image::Rgb;
use std::path::Path;

const PI: f64 = 3.1415926535897932385;
const INF: f64 = f64::INFINITY;

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,

    image_height: u32,
    center: Vector3,
    pixel00_loc: Vector3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32) -> Self {
        let height = ((image_width as f64 / aspect_ratio) as u32).max(1);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let camera_center = Vector3::new(0.0, 0.0, 0.0);

        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / height as f64;

        let viewport_upper_left = camera_center
            - Vector3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            image_height: height,
            center: camera_center,
            pixel00_loc: pixel00_loc,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v,
        }
    }

    pub fn render<T: hittable::Hittable>(&self, world: &T) {
        let black = Rgb([0.0f64, 0.0, 0.0]);
        let mut img = image_handler::Image::new(self.image_width, self.image_height, black);

        // Draw gradient using mutable enumeration
        for (i, j, pixel) in img.enumerate_pixels() {
            print!("\rScanlines remaining: {} ", self.image_height - j);

            let pixel_center =
                self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
            let ray_direction = pixel_center - self.center;

            let r = Ray::new(self.center, ray_direction);
            let color = self.ray_color(&r, world);

            *pixel = Rgb([color[0], color[1], color[2]]);
        }

        println!("\nImage created successfully!");

        // Save to file
        img.save(Path::new("output.png")).expect("Save failed");
    }

    fn ray_color<T: hittable::Hittable>(&self, r: &Ray, world: &T) -> Vector3 {
        let mut rec = hittable::HitRecord::default();
    
        if world.hit(r, Interval::new(0.0, INF), &mut rec) {
            return 0.5 * (rec.normal + Vector3::new(1.0, 1.0, 1.0));
        }
    
        let unit_dir = r.direction().normalized();
        let a = 0.5 * (unit_dir.y() + 1.0);
        (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0)
    }
}
