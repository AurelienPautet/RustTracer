pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod interval;
pub mod camera;
pub mod material;

use rand::Rng;

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::material::{ Dielectric, Lambertian, Material, Metal };
use crate::sphere::Sphere;
use crate::vec3::{ Color, Point3 };
pub use std::f32::{ INFINITY, NEG_INFINITY, consts::PI };
use std::sync::Arc;

fn _degrees_to_radians(degrees: f32) -> f32 {
    (degrees * PI) / 180.0
}

pub fn random_f64() -> f32 {
    rand::random()
}

pub fn random_f64_range(min: f32, max: f32) -> f32 {
    rand::random_range(min..=max)
}
fn main() {
    let mut world = HittableList::new();
    let material_ground: Arc<dyn Material + Send + Sync> = Arc::new(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center: Arc<dyn Material + Send + Sync> = Arc::new(Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    });
    let material_left: Arc<dyn Material + Send + Sync> = Arc::new(Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
        fuzziness: 0.2,
    });
    let material_rigth: Arc<dyn Material + Send + Sync> = Arc::new(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzziness: 1.0,
    });

    let material_front: Arc<dyn Material + Send + Sync> = Arc::new(Dielectric {
        refraction_index: 1.5,
    });
    let material_air_buble: Arc<dyn Material + Send + Sync> = Arc::new(Dielectric {
        refraction_index: 1.0 / 1.33,
    });

    world.add(
        Box::new(Sphere::new(Point3::new(0.0, -1000.5, -1.0), 1000.0, Arc::clone(&material_ground)))
    );
    world.add(
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, Arc::clone(&material_center)))
    );
    world.add(
        Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Arc::clone(&material_air_buble)))
    );
    world.add(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Arc::clone(&material_rigth))));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, 3.5), 0.4, Arc::clone(&material_front))));

    let cam = Camera::new(16.0 / 9.0, 600, 100);
    cam.render(&world);
}
