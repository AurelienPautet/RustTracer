use std::ops::{ Neg, Sub, Add, Mul, Div };

#[derive(Debug, Clone, Copy)]
pub struct Vec3(f64, f64, f64);
pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3(x, y, z)
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
