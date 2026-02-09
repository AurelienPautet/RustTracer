use std::f64::INFINITY;
use std::io::{ self, Write };
use crate::random_f64;
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
    sample_per_pixel: u16,
    pixel_samples_scale: f64,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u16, sample_per_pixel: u16) -> Self {
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

        let pixel_samples_scale = 1.0 / (sample_per_pixel as f64);
        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            sample_per_pixel,
            pixel_samples_scale,
        }
    }

    pub fn render(&self, world: &HittableList) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for y in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - 1 - y);
            io::stderr().flush().unwrap();
            for x in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.sample_per_pixel {
                    let ray = self.get_ray(x, y);
                    pixel_color = pixel_color + Self::ray_color(ray, world);
                }
                write_color(pixel_color * self.pixel_samples_scale);
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

    fn get_ray(&self, x: u16, y: u16) -> Ray {
        let offset = self.sample_square();

        let pixel_center =
            self.pixel00_loc +
            ((x as f64) + offset.x()) * self.pixel_delta_u +
            ((y as f64) + offset.y()) * self.pixel_delta_v;
        let ray_direction = pixel_center - self.center;
        Ray::new(self.center, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }
}
