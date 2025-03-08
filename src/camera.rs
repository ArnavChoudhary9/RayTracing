use crate::hittable;
use crate::image_handler;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utility;
use crate::utility::INF;
use crate::vec3::Vector3;

use image::Rgb;
use std::f64;
use std::path::Path;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    image_height: u32,

    samples_per_pixel: u32,
    pixel_samples_scale: f64,
    max_depth: u32,

    center: Vector3,
    
    pixel00_loc: Vector3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32, max_depth: u32) -> Self {
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

            samples_per_pixel: samples_per_pixel,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
            max_depth: max_depth,

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

            let mut color = Vector3::default();

            for _ in 0..self.samples_per_pixel as i32 {
                let r = self.get_ray(i, j);
                color = color + self.ray_color(&r, self.max_depth, world);
            }
            color = color * self.pixel_samples_scale;

            *pixel = Rgb([color[0], color[1], color[2]]);
        }

        println!("\nImage created successfully!");

        // Save to file
        img.save(Path::new("output.png")).expect("Save failed");
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_disk();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let origin = self.center;
        let direction = pixel_sample - origin;

        Ray::new(origin, direction)
    }

    fn sample_square(&self) -> Vector3 {
        Vector3::new(
            utility::random_double() - 0.5,
            utility::random_double() - 0.5,
            0.0,
        )
    }

    fn sample_disk(&self) -> Vector3 {
        let r = utility::random_double().sqrt();
        let theta = utility::random_double_range(0.0, f64::consts::TAU);
        Vector3::new(r * theta.cos(), r * theta.sin(), 0.0)
    }

    fn ray_color<T: hittable::Hittable>(&self, r: &Ray, depth: u32, world: &T) -> Vector3 {
        if depth <= 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }
        
        let mut rec = hittable::HitRecord::default();

        if world.hit(r, Interval::new(0.0, INF), &mut rec) {
            let direction = Vector3::random_on_hemisphere(&rec.normal);
            return 0.5 * self.ray_color(&Ray::new(rec.p, direction), depth-1, world);
        }

        let unit_dir = r.direction().normalized();
        let a = 0.5 * (unit_dir.y() + 1.0);
        (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0)
    }
}
