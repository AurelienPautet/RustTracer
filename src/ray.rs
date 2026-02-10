use crate::{ vec3::{ Point3, Vec3 } };

#[derive(Debug, Clone, Copy)]
pub struct Ray(Point3, Vec3);

impl Ray {
    pub const fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray(origin, direction)
    }

    pub fn origin(&self) -> Point3 {
        self.0
    }

    pub fn direction(&self) -> Vec3 {
        self.1
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin() + self.direction() * t
    }
}
