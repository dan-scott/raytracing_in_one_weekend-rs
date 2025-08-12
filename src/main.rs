mod color;
mod ray;
mod vec3;

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use color::Color;
use std::io;
use std::io::Write;

fn ray_color(ray: &Ray) -> Color {
    Color::new(0.0, 0.0, 0.0)
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = ((image_width as f64 / aspect_ratio) as u32).max(1);

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::zero();

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for y in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - y);
        for x in 0..image_width {
            let pixel_center =
                pixel00_loc + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray);
            pixel_color.write(&mut io::stdout()).unwrap()
        }
    }
    eprint!("\rDone.                 \n");

    io::stdout().flush().unwrap();
}
