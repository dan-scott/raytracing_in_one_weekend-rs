use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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
        self.e.iter().zip(other.e.iter()).map(|(x, y)| x * y).sum()
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * other.e[2] - self.e[2] * other.e[1],
                self.e[2] * other.e[0] - self.e[0] * other.e[2],
                self.e[0] * other.e[1] - self.e[1] * other.e[0],
            ],
        }
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

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
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
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        for i in 0..3 {
            self.e[i] -= other.e[i];
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
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
