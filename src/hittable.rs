use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point3;
use std::rc::Rc;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Point3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Point3, t: f64) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            front_face: false,
        }
    }

    // Convenience to start a builder from the required initial fields

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Point3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Into<Rc<Box<dyn Hittable>>> for Sphere {
    fn into(self) -> Rc<Box<dyn Hittable>> {
        Rc::new(Box::new(self))
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let cq = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(&cq);
        let c = cq.length_squared() - self.radius.powi(2);

        let discriminant = h.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (h - sqrt_d) / a;
        if !ray_t.contains(root) {
            root = (h + sqrt_d) / a;
            if !ray_t.contains(root) {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;

        // Build the record in two steps to ensure face normal is set properly
        let mut builder = HitRecord::new(p, outward_normal, t);
        builder.set_face_normal(r, &outward_normal);
        Some(builder)
    }
}

pub struct HittableList {
    objects: Vec<Rc<Box<dyn Hittable>>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Rc<Box<dyn Hittable>>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        self.objects.iter().fold(None, |prev, object| {
            let max = prev.clone().map_or(ray_t.max, |prev| prev.t);
            object.hit(r, Interval::new(ray_t.min, max)).or(prev)
        })
    }
}
