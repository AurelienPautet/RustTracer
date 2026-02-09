use std::f64::INFINITY;

use crate::{ hittable::{ HitRecord, Hittable, HittableList }, vec3::{ Color, Point3, Vec3, dot } };

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

    pub fn at(&self, t: f64) -> Point3 {
        self.origin() + self.direction() * t
    }
}

pub fn ray_color(ray: Ray, world: &HittableList) -> Color {
    if let Some(rec) = world.hit(ray, 0.0, INFINITY) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = ray.direction().to_unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

pub fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = center - ray.origin();
    let a = ray.direction().length_squared();
    let h = dot(&ray.direction(), &oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
        return -1.0;
    }
    h - discriminant.sqrt() / a
}
