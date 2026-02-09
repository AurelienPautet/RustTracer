use std::io::{ self, Write };

pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;

use crate::hittable::HittableList;
use crate::ray::{ Ray, ray_color };
use crate::sphere::Sphere;
use crate::vec3::{ Vec3, Point3, Color };
use crate::color::write_color;

pub use std::f64::{ INFINITY, consts::PI };

fn degrees_to_radians(degrees: f64) -> f64 {
    (degrees * PI) / 180.0
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u16 = 400;
    const IMAGE_HEIGHT: u16 = if (((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u16) < 1 {
        1
    } else {
        ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u16
    };

    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = (VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64)) / (IMAGE_HEIGHT as f64);

    const FOCAL_LENGTH: f64 = 1.0;
    const CAMERA_CENTER: Point3 = Point3::new(0.0, 0.0, 0.0);

    const VIEWPORT_U: Vec3 = Vec3::new(VIEWPORT_WIDTH as f64, 0.0, 0.0);
    const VIEWPORT_V: Vec3 = Vec3::new(0.0, -VIEWPORT_HEIGHT as f64, 0.0);

    let PIXEL_DELTA_U: Vec3 = VIEWPORT_U / (IMAGE_WIDTH as f64);
    let PIXEL_DELTA_V: Vec3 = VIEWPORT_V / (IMAGE_HEIGHT as f64);

    let VIEWPORT_UPPER_LEFT: Vec3 =
        CAMERA_CENTER - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - VIEWPORT_U / 2.0 - VIEWPORT_V / 2.0;

    let PIXEL00_LOC: Point3 = VIEWPORT_UPPER_LEFT + 0.5 * (PIXEL_DELTA_U + PIXEL_DELTA_V);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for y in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines remaining: {} ", IMAGE_HEIGHT - 1 - y);
        io::stderr().flush().unwrap();
        for x in 0..IMAGE_WIDTH {
            let pixel_center =
                PIXEL00_LOC + (x as f64) * PIXEL_DELTA_U + (y as f64) * PIXEL_DELTA_V;
            let ray_direction = pixel_center - CAMERA_CENTER;
            let ray = Ray::new(CAMERA_CENTER, ray_direction);
            let pixel_color = ray_color(ray, &world);
            write_color(pixel_color);
        }
    }
}
