use std::f64::INFINITY;
use std::io::{ self, Write };
use minifb::{ Key, Window, WindowOptions };
use rayon::prelude::*;
use std::time::Instant;
use crate::random_f64;
use crate::{
    hittable::{ Hittable, HittableList },
    interval::Interval,
    vec3::{ Color, Point3, Vec3 },
    ray::Ray,
};

pub struct Camera {
    aspect_ratio: f64,
    image_width: u16,
    image_height: u16,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    sample_max: u16,
    sample_current: u16,
    max_depth: u16,
}

pub enum Direction {
    Left,
    Right,
    Forward,
    Backward,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u16, sample_max: u16) -> Self {
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
            sample_max,
            sample_current: 1,
            max_depth: 10,
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.aspect_ratio = (width as f64) / (height as f64);
        self.image_width = width;
        self.image_height = height;
        self.sample_current = 1;
        self.update();
    }

    pub fn update(&mut self) {
        const VIEWPORT_HEIGHT: f64 = 2.0;
        const FOCAL_LENGTH: f64 = 1.0;

        self.image_height = ((self.image_width as f64) / self.aspect_ratio.max(1.0)) as u16;
        let viewport_width =
            (VIEWPORT_HEIGHT * (self.image_width as f64)) / (self.image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn move_camera(&mut self, dir: Direction) {
        let speed = 0.1;
        match dir {
            Direction::Backward => {
                self.center = self.center + Vec3::new(0.0, 0.0, speed);
            }
            Direction::Forward => {
                self.center = self.center + Vec3::new(0.0, 0.0, -speed);
            }
            Direction::Left => {
                self.center = self.center + Vec3::new(-speed, 0.0, 0.0);
            }
            Direction::Right => {
                self.center = self.center + Vec3::new(speed, 0.0, 0.0);
            }
        }
        self.sample_current = 1;
        self.update();
    }

    pub fn render(mut self, world: &HittableList) {
        let mut color_buffer: Vec<Color> =
            vec![Color::new(1.0, 1.0, 1.0); self.image_width as usize * self.image_height as usize];
        let mut buffer: Vec<u32> = vec![0; self.image_width as usize * self.image_height as usize];

        let mut window = Window::new(
            "RustTracer",
            self.image_width as usize,
            self.image_height as usize,
            WindowOptions {
                resize: true,
                ..WindowOptions::default()
            }
        ).unwrap_or_else(|e| {
            panic!("{}", e);
        });

        window.set_target_fps(60);
        print!("render starting");
        let mut fps = 0;
        while window.is_open() && !window.is_key_down(Key::Escape) {
            let start = Instant::now();

            let (new_w, new_h) = window.get_size();
            if new_w != (self.image_width as usize) || new_h != (self.image_height as usize) {
                self.resize(new_w as u16, new_h as u16);
                buffer.resize(new_w * new_h, 0);
                color_buffer.resize(new_w * new_h, Color::new(1.0, 1.0, 1.0));
            }

            if self.sample_current < self.sample_max {
                print!(
                    "\r self.sample_current:{} at a frame rate of: {}",
                    self.sample_current,
                    fps
                );
                io::stdout().flush().unwrap();
                color_buffer
                    .par_iter_mut()
                    .enumerate()
                    .for_each(|(i, pixel)| {
                        let x = i % (self.image_width as usize);
                        let y = i / (self.image_width as usize);

                        let ray = self.get_ray(x as u16, y as u16);
                        let pixel_color = Self::ray_color(ray, self.max_depth, world);
                        if self.sample_current == 1 {
                            *pixel = pixel_color;
                        } else {
                            *pixel = *pixel + pixel_color;
                        }
                    });

                buffer
                    .par_iter_mut()
                    .enumerate()
                    .for_each(|(i, pixel)| {
                        let pixel_color = color_buffer[i] / (self.sample_current as f64);

                        let intensity = Interval::new(0.0, 0.999);

                        let r = (255.99 * intensity.clamp(pixel_color.x().sqrt())) as u32;
                        let g = (255.99 * intensity.clamp(pixel_color.y().sqrt())) as u32;
                        let b = (255.99 * intensity.clamp(pixel_color.z().sqrt())) as u32;

                        *pixel = (r << 16) | (g << 8) | b;
                    });
            }
            if self.sample_current <= self.sample_max {
                self.sample_current += 1;
            }
            if window.is_key_down(Key::W) {
                self.move_camera(Direction::Forward);
            }
            if window.is_key_down(Key::S) {
                self.move_camera(Direction::Backward);
            }
            if window.is_key_down(Key::A) {
                self.move_camera(Direction::Left);
            }
            if window.is_key_down(Key::D) {
                self.move_camera(Direction::Right);
            }

            fps = start.elapsed().as_millis() / (1000 / 60);
            window.update_with_buffer(&buffer, new_w, new_h).unwrap();
        }
    }

    pub fn ray_color(ray: Ray, max_depth: u16, world: &HittableList) -> Color {
        if max_depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(ray, Interval::new(0.001, INFINITY)) {
            let Some((scattered, attenuation)) = rec.mat.scatter(ray, &rec) else {
                return Color::new(0.0, 0.0, 0.0);
            };
            return attenuation * Self::ray_color(scattered, max_depth - 1, world);
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
