pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod interval;
pub mod camera;

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::sphere::Sphere;
use crate::vec3::{ Point3 };

pub use std::f64::{ INFINITY, NEG_INFINITY, consts::PI };

fn degrees_to_radians(degrees: f64) -> f64 {
    (degrees * PI) / 180.0
}

fn main() {
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = Camera::new(16.0 / 9.0, 400);
    cam.render(&world);
}
