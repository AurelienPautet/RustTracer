use std::ops::{ Neg, Sub, Add, Mul, Div };

use crate::{ random_f64, random_f64_range };

#[derive(Debug, Clone, Copy)]
pub struct Vec3(f64, f64, f64);
pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn random() -> Self {
        Self(random_f64(), random_f64(), random_f64())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self(random_f64_range(min, max), random_f64_range(min, max), random_f64_range(min, max))
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let vec = Self::random_range(-1.0, 1.0);
            let lensq = vec.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return vec / lensq;
            }
        }
    }

    pub fn random_on_hemisphere(normal: Self) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if dot(&on_unit_sphere, &normal) > 0.0 {
            return on_unit_sphere;
        }
        -on_unit_sphere
    }

    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn to_unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.0.abs() < s && self.1.abs() < s && self.2.abs() < s
    }

    pub fn reflect(self, n: Self) -> Self {
        self - 2.0 * dot(&self, &n) * n
    }
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3(v1.1 * v2.2 - v1.2 * v2.1, v1.2 * v2.0 - v1.0 * v2.2, v1.0 * v2.1 - v1.1 * v2.0)
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3(-self.x(), -self.y(), -self.z())
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        Vec3(self.x() - other.x(), self.y() - other.y(), self.z() - other.z())
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self::Output {
        Vec3(self.x() + other.x(), self.y() + other.y(), self.z() + other.z())
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Self::Output {
        Vec3(self.x() * other.x(), self.y() * other.y(), self.z() * other.z())
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, constant: f64) -> Self::Output {
        Vec3(self.x() * constant, self.y() * constant, self.z() * constant)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, constant: f64) -> Self::Output {
        self * (1.0 / constant)
    }
}
