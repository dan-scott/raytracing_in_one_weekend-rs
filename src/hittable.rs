use crate::ray::Ray;
use crate::vec3::Point3;

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
    pub fn builder(p: Point3, normal: Point3, t: f64) -> HitRecordBuilder {
        HitRecordBuilder::new(p, normal, t)
    }
    
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Point3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

// A linear/type-state style builder for HitRecord.
// First provide p, normal, t. Then provide the ray and outward normal to finalize.
pub struct HitRecordBuilder {
    p: Point3,
    normal: Point3,
    t: f64,
}

impl HitRecordBuilder {
    pub fn new(p: Point3, normal: Point3, t: f64) -> Self {
        HitRecordBuilder { p, normal, t }
    }

    // Consumes the builder to create a HitRecord and set the face-normal correctly.
    pub fn with_ray(self, r: &Ray, outward_normal: &Point3) -> HitRecord {
        let mut rec = HitRecord::new(self.p, self.normal, self.t);
        rec.set_face_normal(r, outward_normal);
        rec
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
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

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_min: f64, ray_max: f64) -> Option<HitRecord> {
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
        let range = ray_min..ray_max;
        if !range.contains(&root) {
            root = (h + sqrt_d) / a;
            if !range.contains(&root) {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;

        // Build the record in two steps to ensure face normal is set properly
        let builder = HitRecord::builder(p, outward_normal, t);
        Some(builder.with_ray(r, &outward_normal))
    }
}
