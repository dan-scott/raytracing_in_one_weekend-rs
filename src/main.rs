mod color;
mod hittable;
mod ray;
mod vec3;
mod interval;
mod camera;
mod utils;

use crate::hittable::{Hittable, HittableList, Sphere};
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
use crate::camera::Camera;
use crate::interval::Interval;

#[cfg(not(target_os = "windows"))]
fn get_out_stream() -> Stdout {
    stdout()
}

#[cfg(target_os = "windows")]
fn get_out_stream() -> File {
    let path = args().nth(1).unwrap_or_else(|| "output.ppm".to_string());
    File::create(path).unwrap()
}

fn main() {
    let mut world = HittableList::new();
    world.add(Sphere::new((0.0, 0.0, -1.0).into(), 0.5).into());
    world.add(Sphere::new((0.0, -100.5, -1.0).into(), 100.0).into());


    let mut out_stream = get_out_stream();

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;

    cam.render(&world, &mut out_stream).unwrap();
    
    out_stream.flush().unwrap();
}
