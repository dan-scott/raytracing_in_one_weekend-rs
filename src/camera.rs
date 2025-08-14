use crate::color::Color;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use rand::Rng;
use std::io::Write;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,

    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            pixel_samples_scale: 1.0,
            image_height: 0,
            center: Point3::zero(),
            pixel00_loc: Point3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: &dyn Hittable, out: &mut impl Write) -> anyhow::Result<()> {
        self.initialize();

        write!(out, "P3\n{} {}\n255\n", self.image_width, self.image_height)?;

        for y in 0..self.image_height {
            eprint!("\rScanlines remaining: {:>4}", self.image_height - y);
            for x in 0..self.image_width {
                let mut pixel_color = Color::zero();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x as f64, y as f64);
                    pixel_color += Camera::ray_color(&ray, world);
                }

                pixel_color.write(self.pixel_samples_scale, out)?
            }
        }

        eprintln!("\rDone.                 \n");
        Ok(())
    }
}

impl Camera {
    fn get_ray(&self, x: f64, y: f64) -> Ray {
        let offset = Camera::sample_offset();
        let pixel_sample = self.pixel00_loc
            + ((x + offset.x()) * self.pixel_delta_u)
            + ((y + offset.y()) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_offset() -> Vec3 {
        let mut rng = rand::rng();
        Vec3::new(
            rng.random::<f64>() - 0.5,
            rng.random::<f64>() - 0.5,
            0.0,
        )
    }

    fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
        if let Some(rec) = world.hit(r, Interval::new(0.0, f64::INFINITY)) {
            0.5 * (rec.normal + (1.0, 1.0, 1.0).into())
        } else {
            let unit_direction = r.direction().unit_vector();
            let a = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio).max(1.0) as u32;
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        self.center = Point3::zero();

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }
}
