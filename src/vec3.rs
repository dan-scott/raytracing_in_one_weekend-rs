use rand::Rng;
use rand::prelude::ThreadRng;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign};
use std::thread::Thread;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3],
}

pub use Vec3 as Point3;

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn random(rng: &mut ThreadRng) -> Vec3 {
        Self::new(rng.random(), rng.random(), rng.random())
    }

    pub fn random_range(rng: &mut ThreadRng, range: Range<f64>) -> Vec3 {
        Self::new(
            rng.random_range(range.clone()),
            rng.random_range(range.clone()),
            rng.random_range(range),
        )
    }

    pub fn random_unit_vector(rng: &mut ThreadRng) -> Vec3 {
        loop {
            let p = Self::random_range(rng, -1.0..1.0);
            let len_sq = p.length_squared();
            if 1e-160 < len_sq && len_sq <= 1.0 {
                return p / len_sq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(rng: &mut ThreadRng, normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector(rng);
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn tuple(&self) -> (f64, f64, f64) {
        (self.e[0], self.e[1], self.e[2])
    }

    pub fn length_squared(&self) -> f64 {
        self.e.iter().map(|x| x * x).sum()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        (
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            self.e[2] * other.e[0] - self.e[0] * other.e[2],
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
        )
            .into()
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
}

impl From<[f64; 3]> for Vec3 {
    fn from(e: [f64; 3]) -> Vec3 {
        Vec3 { e }
    }
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from(t: (f64, f64, f64)) -> Vec3 {
        Self::new(t.0, t.1, t.2)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        (
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
            .into()
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        for i in 0..3 {
            self.e[i] += other.e[i];
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        (
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
            .into()
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        for i in 0..3 {
            self.e[i] -= other.e[i];
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        (-self.e[0], -self.e[1], -self.e[2]).into()
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        (
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
            .into()
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        for i in 0..3 {
            self.e[i] *= other;
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        (1.0 / other) * self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        for i in 0..3 {
            self.e[i] /= other;
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-12;

    fn approx_eq(a: f64, b: f64) {
        assert!(
            (a - b).abs() <= EPS,
            "expected {} ≈ {} (|diff|={})",
            a,
            b,
            (a - b).abs()
        );
    }

    fn vec_approx_eq(a: &Vec3, b: &Vec3) {
        approx_eq(a.x(), b.x());
        approx_eq(a.y(), b.y());
        approx_eq(a.z(), b.z());
    }

    #[test]
    fn constructors_and_accessors() {
        let z = Vec3::zero();
        assert_eq!(z.tuple(), (0.0, 0.0, 0.0));
        approx_eq(z.length(), 0.0);

        let v: Vec3 = (1.0, -2.5, 3.25).into();
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), -2.5);
        assert_eq!(v.z(), 3.25);
        assert_eq!(v.tuple(), (1.0, -2.5, 3.25));

        let w: Vec3 = [4.0, 5.0, 6.0].into();
        assert_eq!(w.tuple(), (4.0, 5.0, 6.0));

        let t: Vec3 = (7.0, 8.0, 9.0).into();
        assert_eq!(t.tuple(), (7.0, 8.0, 9.0));
    }

    #[test]
    fn length_and_unit_vector() {
        let v: Vec3 = (3.0, 4.0, 12.0).into();
        assert_eq!(v.length_squared(), 9.0 + 16.0 + 144.0);
        approx_eq(v.length(), 13.0);

        let u = v.unit_vector();
        approx_eq(u.length(), 1.0);
        // u should be v normalized
        vec_approx_eq(&u, &((1.0 / 13.0) * v));

        // unit vector of unit vector is itself
        vec_approx_eq(&u.unit_vector(), &u);
    }

    #[test]
    fn dot_product_properties() {
        let a: Vec3 = (1.0, 2.0, 3.0).into();
        let b: Vec3 = (-4.0, 5.0, -6.0).into();
        let c: Vec3 = (7.5, -8.25, 0.5).into();

        // numeric value
        approx_eq(a.dot(&b), -12.0);

        // symmetry
        approx_eq(a.dot(&b), b.dot(&a));

        // linearity: (a+b)·c = a·c + b·c
        let left = (a + b).dot(&c);
        let right = a.dot(&c) + b.dot(&c);
        approx_eq(left, right);

        // relation with length: a·a = |a|^2
        approx_eq(a.dot(&a), a.length_squared());
    }

    #[test]
    fn cross_product_properties() {
        let i: Vec3 = (1.0, 0.0, 0.0).into();
        let j: Vec3 = (0.0, 1.0, 0.0).into();
        let k: Vec3 = (0.0, 0.0, 1.0).into();

        vec_approx_eq(&i.cross(&j), &k); // right-hand rule
        vec_approx_eq(&j.cross(&k), &i);
        vec_approx_eq(&k.cross(&i), &j);

        // orthogonality: a×b ⟂ a and b
        let a: Vec3 = (2.0, -3.0, 4.0).into();
        let b: Vec3 = (-1.0, 5.0, 2.0).into();
        let c = a.cross(&b);
        approx_eq(c.dot(&a), 0.0);
        approx_eq(c.dot(&b), 0.0);

        // numeric value
        assert_eq!(c.tuple(), (-26.0, -8.0, 7.0));

        // anti-commutativity: a×b = -(b×a)
        vec_approx_eq(&c, &(-1.0 * b.cross(&a)));
    }

    #[test]
    fn addition_and_subtraction() {
        let a: Vec3 = (1.0, 2.0, 3.0).into();
        let b: Vec3 = (4.0, -5.0, 6.0).into();
        assert_eq!((a + b).tuple(), (5.0, -3.0, 9.0));
        assert_eq!((a - b).tuple(), (-3.0, 7.0, -3.0));

        // AddAssign and SubAssign
        let mut c = a;
        c += b;
        assert_eq!(c.tuple(), (5.0, -3.0, 9.0));
        c -= b;
        assert_eq!(c.tuple(), a.tuple());

        // commutativity of addition
        vec_approx_eq(&(a + b), &(b + a));
    }

    #[test]
    fn element_wise_and_scalar_multiplication() {
        let a: Vec3 = (2.0, -3.0, 4.0).into();
        let b: Vec3 = (-5.0, 6.0, 0.5).into();
        assert_eq!((a * b).tuple(), (-10.0, -18.0, 2.0));

        let s = -2.5;
        assert_eq!((a * s).tuple(), (a.x() * s, a.y() * s, a.z() * s));
        assert_eq!((s * a).tuple(), (a.x() * s, a.y() * s, a.z() * s));

        let mut c = a;
        c *= s;
        assert_eq!(c.tuple(), (a.x() * s, a.y() * s, a.z() * s));

        // distributivity s*(a+b) = s*a + s*b
        let left = s * (a + b);
        let right = s * a + s * b;
        vec_approx_eq(&left, &right);
    }

    #[test]
    fn scalar_division() {
        let a: Vec3 = (9.0, -3.0, 12.0).into();
        let d = 3.0;
        assert_eq!((a / d).tuple(), (3.0, -1.0, 4.0));

        let mut b = a;
        b /= d;
        assert_eq!(b.tuple(), (3.0, -1.0, 4.0));

        // (a/d) * d == a
        vec_approx_eq(&((a / d) * d), &a);
    }

    #[test]
    fn display_formatting() {
        let v: Vec3 = (1.5, -2.0, 3.25).into();
        let s = format!("{}", v);
        assert_eq!(s, "1.5 -2 3.25");
    }
}
