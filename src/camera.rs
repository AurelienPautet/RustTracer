use std::f32::INFINITY;
use rayon::prelude::*;
use crate::vec3::cross;
use crate::{ _degrees_to_radians, Size, random_f32 };
use crate::{
    hittable::{ Hittable, HittableList },
    interval::Interval,
    vec3::{ Color, Point3, Vec3 },
    ray::Ray,
};

#[derive(Clone, Copy)]
struct RayGenParams {
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    center: Point3,
    defocus_angle: f32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl RayGenParams {
    fn get_ray(&self, x: u16, y: u16) -> Ray {
        let offset = Vec3::new(random_f32() - 0.5, random_f32() - 0.5, 0.0);

        let pixel_center =
            self.pixel00_loc +
            ((x as f32) + offset.x()) * self.pixel_delta_u +
            ((y as f32) + offset.y()) * self.pixel_delta_v;
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            let p = Vec3::random_in_unit_disk();
            self.center + p.x() * self.defocus_disk_u + p.y() * self.defocus_disk_v
        };
        let ray_direction = pixel_center - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }
}

pub struct Camera {
    pub fov: f32,
    pub defocus_angle: f32,
    pub focus_dist: f32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    lookfrom: Point3,
    vup: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    image_size: Size,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    sample_max: u16,
    pub sample_current: u16,
    high_res_max_depth: u16,
    low_res_max_depth: u16,
    yaw: f32,
    pitch: f32,
    sample_ratio: u16,
    color_buffer: Vec<Color>,
    full_res_count: u32,
}

pub enum Direction {
    Left,
    Right,
    Forward,
    Backward,
}

impl Camera {
    pub fn new(fov: f32, size: Size, sample_max: u16, sample_ratio: u16) -> Self {
        const ZERO: Vec3 = Vec3::new(0.0, 0.0, 0.0);

        let mut res = Self {
            fov,
            defocus_angle: 0.0,
            focus_dist: 3.4,
            defocus_disk_u: ZERO,
            defocus_disk_v: ZERO,
            lookfrom: ZERO,
            vup: Vec3::new(0.0, 1.0, 0.0),
            u: Vec3::new(1.0, 0.0, 0.0),
            v: Vec3::new(0.0, 1.0, 0.0),
            w: Vec3::new(0.0, 0.0, 1.0),
            image_size: size,
            center: Vec3::new(0.0, 1.0, 30.0),
            pixel00_loc: ZERO,
            pixel_delta_u: ZERO,
            pixel_delta_v: ZERO,
            sample_max,
            sample_current: 0,
            high_res_max_depth: 50,
            low_res_max_depth: 10,
            yaw: -90.0,
            pitch: 0.0,
            sample_ratio,
            color_buffer: vec![Color::new(0.0, 0.0, 0.0); size.area()],
            full_res_count: 0,
        };
        res.update();
        res
    }

    pub fn resize(&mut self, size: Size) {
        self.image_size = size;
        self.color_buffer.resize(size.area(), Color::new(0.0, 0.0, 0.0));
        self.clear();
    }

    pub fn clear(&mut self) {
        self.color_buffer.fill(Color::new(0.0, 0.0, 0.0));
        self.full_res_count = 0;
        self.sample_current = 0;
        self.update();
    }

    pub fn update(&mut self) {
        let radius = self.yaw.to_radians();
        let pitch = self.pitch.to_radians();

        let front = Vec3::new(
            radius.cos() * pitch.cos(),
            pitch.sin(),
            radius.sin() * pitch.cos()
        ).to_unit_vector();

        self.w = -front;
        self.u = cross(&self.vup, &self.w).to_unit_vector();
        self.v = cross(&self.w, &self.u);

        self.lookfrom = self.center;

        let theta = _degrees_to_radians(self.fov);
        let h = (theta / 2.0).tan();
        let viewport_heigth = 2.0 * h * self.focus_dist;
        let viewport_width =
            (viewport_heigth * (self.image_size.w as f32)) / (self.image_size.h as f32);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_heigth * -self.v;

        self.pixel_delta_u = viewport_u / (self.image_size.w as f32);
        self.pixel_delta_v = viewport_v / (self.image_size.h as f32);

        let viewport_upper_left =
            self.center - self.focus_dist * self.w - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * _degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    pub fn move_camera(&mut self, dir: Direction) {
        let speed = 0.1;
        match dir {
            Direction::Backward => {
                self.center = self.center + speed * self.w;
            }
            Direction::Forward => {
                self.center = self.center + -speed * self.w;
            }
            Direction::Left => {
                self.center = self.center + -speed * self.u;
            }
            Direction::Right => {
                self.center = self.center + speed * self.u;
            }
        }
        self.clear();
        self.update();
    }

    pub fn rotate_camera(&mut self, dx: f32, dy: f32) {
        let sensitivity = 0.3;
        self.yaw += dx * sensitivity;
        self.pitch += dy * sensitivity;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        self.sample_current = 0;
        self.clear();
        self.update();
    }

    pub fn render(&mut self, world: &HittableList, buffer: &mut Vec<u32>) {
        if self.sample_current < self.sample_max {
            let ratio = self.sample_ratio as usize;
            let block_size: usize = (ratio >> (self.sample_current as usize)).max(1);

            let params: RayGenParams = RayGenParams {
                pixel00_loc: self.pixel00_loc,
                pixel_delta_u: self.pixel_delta_u,
                pixel_delta_v: self.pixel_delta_v,
                center: self.center,
                defocus_angle: self.defocus_angle,
                defocus_disk_u: self.defocus_disk_u,
                defocus_disk_v: self.defocus_disk_v,
            };
            if block_size > 1 && self.full_res_count == 0 {
                let cols = (self.image_size.w + block_size - 1) / block_size;
                let rows = (self.image_size.h + block_size - 1) / block_size;
                let total_blocks = cols * rows;

                let max_depth = self.low_res_max_depth;

                let block_colors: Vec<u32> = (0..total_blocks)
                    .into_par_iter()
                    .map(|bi| {
                        let bx = bi % cols;
                        let by = bi / cols;
                        let px = (bx * block_size + block_size / 2).min(self.image_size.w - 1);
                        let py = (by * block_size + block_size / 2).min(self.image_size.h - 1);

                        let ray = params.get_ray(px as u16, py as u16);
                        let pixel_color = Self::ray_color(&ray, max_depth, world);

                        let intensity = Interval::new(0.0, 0.999);
                        let r = (255.99 * intensity.clamp(pixel_color.x().sqrt())) as u32;
                        let g = (255.99 * intensity.clamp(pixel_color.y().sqrt())) as u32;
                        let b = (255.99 * intensity.clamp(pixel_color.z().sqrt())) as u32;
                        (r << 16) | (g << 8) | b
                    })
                    .collect();

                for bi in 0..total_blocks {
                    let bx = bi % cols;
                    let by = bi / cols;
                    let color = block_colors[bi];
                    for dy in 0..block_size {
                        for dx in 0..block_size {
                            let px = bx * block_size + dx;
                            let py = by * block_size + dy;
                            if px < self.image_size.w && py < self.image_size.h {
                                buffer[py * self.image_size.w + px] = color;
                            }
                        }
                    }
                }

                self.sample_current += 1;
            } else {
                let max_depth = self.high_res_max_depth;

                self.color_buffer
                    .par_iter_mut()
                    .enumerate()
                    .for_each(|(i, pixel)| {
                        let x = i % self.image_size.w;
                        let y = i / self.image_size.w;
                        let ray = params.get_ray(x as u16, y as u16);
                        let pixel_color = Self::ray_color(&ray, max_depth, world);
                        *pixel = *pixel + pixel_color;
                    });
                self.full_res_count += 1;
                self.sample_current += 1;

                let inv_samples: f32 = 1.0 / (self.full_res_count as f32);
                buffer
                    .par_iter_mut()
                    .enumerate()
                    .for_each(|(i, pixel)| {
                        let pixel_color = self.color_buffer[i] * inv_samples;
                        let intensity = Interval::new(0.0, 0.999);
                        let r = (255.99 * intensity.clamp(pixel_color.x().sqrt())) as u32;
                        let g = (255.99 * intensity.clamp(pixel_color.y().sqrt())) as u32;
                        let b = (255.99 * intensity.clamp(pixel_color.z().sqrt())) as u32;
                        *pixel = (r << 16) | (g << 8) | b;
                    });
            }
        } else {
            let inv_samples = 1.0 / (self.full_res_count as f32).max(1.0);
            buffer
                .par_iter_mut()
                .enumerate()
                .for_each(|(i, pixel)| {
                    let pixel_color = self.color_buffer[i] * inv_samples;
                    let intensity = Interval::new(0.0, 0.999);
                    let r = (255.99 * intensity.clamp(pixel_color.x().sqrt())) as u32;
                    let g = (255.99 * intensity.clamp(pixel_color.y().sqrt())) as u32;
                    let b = (255.99 * intensity.clamp(pixel_color.z().sqrt())) as u32;
                    *pixel = (r << 16) | (g << 8) | b;
                });
        }
    }

    pub fn ray_color(ray: &Ray, max_depth: u16, world: &HittableList) -> Color {
        if max_depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(&ray, Interval::new(0.001, INFINITY)) {
            let Some((scattered, attenuation)) = rec.mat.scatter(&ray, &rec) else {
                return Color::new(0.0, 0.0, 0.0);
            };
            return attenuation * Self::ray_color(&scattered, max_depth - 1, world);
        }
        let unit_direction = ray.direction().to_unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
