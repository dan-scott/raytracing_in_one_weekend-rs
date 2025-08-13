mod color;
mod hittable;
mod ray;
mod vec3;

use crate::hittable::{Hittable, HittableList};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use color::Color;
#[cfg(target_os = "windows")]
use std::env::args;
#[cfg(target_os = "windows")]
use std::fs::File;
use std::io::Write;
#[cfg(not(target_os = "windows"))]
use std::io::{Stdout, stdout};
use std::rc::Rc;

#[cfg(not(target_os = "windows"))]
fn get_out_stream() -> Stdout {
    stdout()
}

#[cfg(target_os = "windows")]
fn get_out_stream() -> File {
    let path = args().nth(1).unwrap_or_else(|| "output.ppm".to_string());
    File::create(path).unwrap()
}

fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        return 0.5 * (hit.normal + Vec3::new(1.0, 1.0, 1.0));
    }
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = ((image_width as f64 / aspect_ratio) as u32).max(1);

    let mut world = HittableList::new();
    world.add(Rc::new(Box::new(hittable::Sphere::new(
        (0.0, 0.0, -1.0).into(),
        0.5,
    ))));
    world.add(Rc::new(Box::new(hittable::Sphere::new(
        (0.0, -100.5, -1.0).into(),
        100.0,
    ))));

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::zero();

    let viewport_u: Vec3 = (viewport_width, 0.0, 0.0).into();
    let viewport_v: Vec3 = (0.0, -viewport_height, 0.0).into();

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - (0.0, 0.0, focal_length).into() - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut out_stream = get_out_stream();

    write!(out_stream, "P3\n{} {}\n255\n", image_width, image_height).unwrap();

    for y in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - y);
        for x in 0..image_width {
            let pixel_center =
                pixel00_loc + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray, &world);
            pixel_color.write(&mut out_stream).unwrap()
        }
    }
    eprint!("\rDone.                 \n");
    out_stream.flush().unwrap();
}
