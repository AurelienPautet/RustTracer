pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod interval;
pub mod camera;
pub mod material;

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

pub fn random_f32() -> f32 {
    rand::random()
}

pub fn random_f32_range(min: f32, max: f32) -> f32 {
    rand::random_range(min..=max)
}
fn main() {
    let mut world = HittableList::new();

    let ground_material: Arc<dyn Material + Send + Sync> = Arc::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.add(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -6..6 {
        for b in -6..6 {
            let choose_mat = random_f32();
            let center = Point3::new(
                (a as f32) + 0.9 * random_f32(),
                0.2,
                (b as f32) + 0.9 * random_f32()
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material + Send + Sync>;

                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian { albedo });
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_f32_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal { albedo, fuzziness: fuzz });
                } else {
                    sphere_material = Arc::new(Dielectric {
                        refraction_index: random_f32_range(0.5, 2.5),
                        frostedness: random_f32_range(0.0, 0.05),
                    });
                }

                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1: Arc<dyn Material + Send + Sync> = Arc::new(Dielectric {
        refraction_index: 1.5,
        frostedness: 0.0,
    });
    world.add(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2: Arc<dyn Material + Send + Sync> = Arc::new(Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    world.add(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3: Arc<dyn Material + Send + Sync> = Arc::new(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzziness: 0.0,
    });
    world.add(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    let cam = Camera::new(16.0 / 9.0, 20.0, 600, 500, 4);
    cam.render(&world);
}
