mod image_handler;
mod vec3;
mod ray;

use vec3::Vector3 as Vector3;
use ray::Ray as Ray;

use std::path::Path;
use image::Rgb;

fn hit_sphere(center: &Vector3, radius: f64, r: &Ray) -> f64 {
    let oc = *center - r.origin();
    let a = r.direction().squared_length();
    let h = oc.dot(&r.direction());
    let c = oc.squared_length() - radius * radius;
    let discriminant = h*h -  a*c;

    if discriminant < 0.0 {
        return -1.0;
    }
    (h - discriminant.sqrt()) / a
}

fn ray_color(r: &Ray) -> Vector3 {
    let t = hit_sphere(&Vector3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let N = (r.at(t) - Vector3::new(0.0, 0.0, -1.0)).normalized();
        return 0.5*Vector3::new(N.x()+1.0, N.y()+1.0, N.z()+1.0);
    }

    let unit_dir = r.direction().normalized();
    let a = 0.5 * (unit_dir.y() + 1.0);
    (1.0-a)*Vector3::new(1.0, 1.0, 1.0) + a*Vector3::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width: u32 = 1280;

    let height = ((width as f64 / aspect_ratio) as u32).max(1);

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let camera_center = Vector3::new(0.0, 0.0, 0.0);

    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / width as f64;
    let pixel_delta_v = viewport_v / height as f64;

    let viewport_upper_left = camera_center - Vector3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let black = Rgb([0.0f64, 0.0, 0.0]);
    let mut img = image_handler::Image::new(width, height, black);

    // Draw gradient using mutable enumeration
    for (i, j, pixel) in img.enumerate_pixels() {
        print!("\rScanlines remaining: {} ", height - j);

        let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
        let ray_direction = pixel_center - camera_center;

        let r = Ray::new(camera_center, ray_direction);
        let color = ray_color(&r);

        *pixel = Rgb([
            color[0], color[1], color[2]
        ]);
    }

    println!("\nImage created successfully!");

    // Save to file
    img.save(Path::new("output.png")).expect("Save failed");
}
