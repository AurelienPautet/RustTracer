use std::f64::INFINITY;
use std::io::{ self, Write };
use crate::{
    hittable::{ Hittable, HittableList },
    interval::Interval,
    vec3::{ Color, Point3, Vec3 },
    ray::Ray,
    color::write_color,
};

pub struct Camera {
    aspect_ratio: f64,
    image_width: u16,
    image_height: u16,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u16) -> Self {
        const VIEWPORT_HEIGHT: f64 = 2.0;
        const FOCAL_LENGTH: f64 = 1.0;

        let image_height = ((image_width as f64) / aspect_ratio).max(1.0) as u16;
        let viewport_width = (VIEWPORT_HEIGHT * (image_width as f64)) / (image_height as f64);

        let center = Point3::new(0.0, 0.0, 0.0);
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &HittableList) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for y in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - 1 - y);
            io::stderr().flush().unwrap();
            for x in 0..self.image_width {
                let pixel_center =
                    self.pixel00_loc +
                    (x as f64) * self.pixel_delta_u +
                    (y as f64) * self.pixel_delta_v;
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);
                let pixel_color = Self::ray_color(ray, world);
                write_color(pixel_color);
            }
        }
    }

    pub fn ray_color(ray: Ray, world: &HittableList) -> Color {
        if let Some(rec) = world.hit(ray, Interval::new(0.0, INFINITY)) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }
        let unit_direction = ray.direction().to_unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
