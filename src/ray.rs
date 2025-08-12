use crate::vec3::{Point3, Vec3};

pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn zero() -> Ray {
        Ray {
            origin: Point3::zero(),
            dir: Vec3::zero(),
        }
    }

    pub fn new(origin: Point3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.dir * t
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }
}
