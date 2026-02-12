use std::f32::INFINITY;
use std::io::{ self, Write };
use minifb::{ Key, Window, WindowOptions };
use rayon::prelude::*;
use std::time::Instant;
use crate::vec3::cross;
use crate::{ _degrees_to_radians, random_f32 };
use crate::{
    hittable::{ Hittable, HittableList },
    interval::Interval,
    vec3::{ Color, Point3, Vec3 },
    ray::Ray,
};

pub struct Camera {
    aspect_ratio: f32,
    fov: f32,
    defocus_angle: f32,
    focus_dist: f32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    lookfrom: Point3,
    vup: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    image_width: u16,
    image_height: u16,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    sample_max: u16,
    sample_current: u16,
    max_depth: u16,
    yaw: f32,
    pitch: f32,
    sample_ratio: u16,
    is_moving: bool,
}

pub enum Direction {
    Left,
    Right,
    Forward,
    Backward,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        fov: f32,
        image_width: u16,
        sample_max: u16,
        sample_ratio: u16
    ) -> Self {
        const ZERO: Vec3 = Vec3::new(0.0, 0.0, 0.0);

        let mut res = Self {
            aspect_ratio,
            fov,
            defocus_angle: 0.6,
            focus_dist: 3.4,
            defocus_disk_u: ZERO,
            defocus_disk_v: ZERO,
            lookfrom: ZERO,
            vup: Vec3::new(0.0, 1.0, 0.0),
            u: Vec3::new(1.0, 0.0, 0.0),
            v: Vec3::new(0.0, 1.0, 0.0),
            w: Vec3::new(0.0, 0.0, 1.0),
            image_width,
            image_height: 0,
            center: Vec3::new(0.0, 1.0, 3.0),
            pixel00_loc: ZERO,
            pixel_delta_u: ZERO,
            pixel_delta_v: ZERO,
            sample_max,
            sample_current: 0,
            max_depth: 50,
            yaw: -90.0,
            pitch: 0.0,
            sample_ratio,
            is_moving: false,
        };
        res.update();
        res
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.aspect_ratio = (width as f32) / (height as f32);
        self.image_width = width;
        self.image_height = height;
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
        self.image_height = ((self.image_width as f32) / self.aspect_ratio.max(1.0)) as u16;
        let viewport_width =
            (viewport_heigth * (self.image_width as f32)) / (self.image_height as f32);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_heigth * -self.v;

        self.pixel_delta_u = viewport_u / (self.image_width as f32);
        self.pixel_delta_v = viewport_v / (self.image_height as f32);

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
        self.sample_current = 0;
        self.is_moving = true;
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
        self.is_moving = true;
        self.update();
    }

    pub fn render(mut self, world: &HittableList) {
        let mut color_buffer: Vec<Color> =
            vec![Color::new(0.0, 0.0, 0.0); self.image_width as usize * self.image_height as usize];
        let mut buffer: Vec<u32> = vec![0; self.image_width as usize * self.image_height as usize];

        let mut last_mouse_pos: (f32, f32) = (0.0, 0.0);
        let mut first_mouse = true;

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

        window.set_target_fps(200);
        let mut fps = 0;
        let mut full_res_count: u32 = 0;
        while window.is_open() && !window.is_key_down(Key::Escape) {
            let start = Instant::now();
            let static_max_depth = self.max_depth;
            self.is_moving = false;

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
            if window.is_key_down(Key::F) {
                self.fov = self.fov + 1.0;
                color_buffer.fill(Color::new(0.0, 0.0, 0.0));
                full_res_count = 0;
                self.sample_current = 0;
                self.update();
            }
            if window.is_key_down(Key::V) {
                self.fov = self.fov - 1.0;
                color_buffer.fill(Color::new(0.0, 0.0, 0.0));
                full_res_count = 0;
                self.sample_current = 0;
                self.update();
            }
            if window.is_key_down(Key::G) {
                self.focus_dist = self.focus_dist + 0.1;
                color_buffer.fill(Color::new(0.0, 0.0, 0.0));
                full_res_count = 0;
                self.sample_current = 0;
                self.update();
            }
            if window.is_key_down(Key::B) {
                self.focus_dist = self.focus_dist - 0.1;
                color_buffer.fill(Color::new(0.0, 0.0, 0.0));
                full_res_count = 0;
                self.sample_current = 0;
                self.update();
            }
            if window.is_key_down(Key::H) {
                self.defocus_angle = self.defocus_angle - 0.1;
                color_buffer.fill(Color::new(0.0, 0.0, 0.0));
                full_res_count = 0;
                self.sample_current = 0;
                self.update();
            }
            if window.is_key_down(Key::N) {
                self.defocus_angle = self.defocus_angle + 0.1;
                color_buffer.fill(Color::new(0.0, 0.0, 0.0));
                full_res_count = 0;
                self.sample_current = 0;
                self.update();
            }

            if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(minifb::MouseMode::Pass) {
                if first_mouse {
                    last_mouse_pos = (mouse_x, mouse_y);
                    first_mouse = false;
                }

                let x_offset = mouse_x - last_mouse_pos.0;
                let y_offset = last_mouse_pos.1 - mouse_y;
                last_mouse_pos = (mouse_x, mouse_y);
                let min = 0.01;
                if window.get_mouse_down(minifb::MouseButton::Left) {
                    if x_offset.abs() > min || y_offset.abs() > min {
                        self.rotate_camera(x_offset, y_offset);
                    }
                }
            }

            let (new_w, new_h) = window.get_size();
            if new_w != (self.image_width as usize) || new_h != (self.image_height as usize) {
                self.resize(new_w as u16, new_h as u16);
                buffer.resize(new_w * new_h, 0);
                color_buffer.resize(new_w * new_h, Color::new(0.0, 0.0, 0.0));
                color_buffer.fill(Color::new(0.0, 0.0, 0.0));
                full_res_count = 0;
            }

            if self.is_moving {
                color_buffer.fill(Color::new(0.0, 0.0, 0.0));
                full_res_count = 0;
                self.max_depth = 10;
            } else {
                if static_max_depth != self.max_depth {
                    self.max_depth = static_max_depth;
                }
            }

            if self.sample_current < self.sample_max {
                print!("\r sample:{} fps:{}              ", self.sample_current, fps);
                io::stdout().flush().unwrap();
                let title =
                    String::from("RustTracer, fps :") +
                    &fps.to_string() +
                    &String::from(" sample :") +
                    &self.sample_current.to_string();
                window.set_title(&title);

                let ratio = self.sample_ratio as usize;
                let w = self.image_width as usize;
                let h = self.image_height as usize;
                let block_size: usize = (ratio >> (self.sample_current as usize)).max(1);

                if block_size > 1 && full_res_count == 0 {
                    let cols = (w + block_size - 1) / block_size;
                    let rows = (h + block_size - 1) / block_size;
                    let total_blocks = cols * rows;

                    let block_colors: Vec<u32> = (0..total_blocks)
                        .into_par_iter()
                        .map(|bi| {
                            let bx = bi % cols;
                            let by = bi / cols;
                            let px = (bx * block_size + block_size / 2).min(w - 1);
                            let py = (by * block_size + block_size / 2).min(h - 1);

                            let ray = self.get_ray(px as u16, py as u16);
                            let pixel_color = Self::ray_color(ray, self.max_depth, world);

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
                                if px < w && py < h {
                                    buffer[py * w + px] = color;
                                }
                            }
                        }
                    }

                    if !self.is_moving {
                        self.sample_current += 1;
                    }
                } else {
                    color_buffer
                        .par_iter_mut()
                        .enumerate()
                        .for_each(|(i, pixel)| {
                            let x = i % w;
                            let y = i / w;
                            let ray = self.get_ray(x as u16, y as u16);
                            let pixel_color = Self::ray_color(ray, self.max_depth, world);
                            *pixel = *pixel + pixel_color;
                        });

                    full_res_count += 1;

                    buffer
                        .par_iter_mut()
                        .enumerate()
                        .for_each(|(i, pixel)| {
                            let pixel_color = color_buffer[i] / (full_res_count as f32);
                            let intensity = Interval::new(0.0, 0.999);
                            let r = (255.99 * intensity.clamp(pixel_color.x().sqrt())) as u32;
                            let g = (255.99 * intensity.clamp(pixel_color.y().sqrt())) as u32;
                            let b = (255.99 * intensity.clamp(pixel_color.z().sqrt())) as u32;
                            *pixel = (r << 16) | (g << 8) | b;
                        });

                    self.sample_current += 1;
                }
            }

            let elapsed_ms = start.elapsed().as_millis();
            fps = if elapsed_ms > 0 { 1000 / (elapsed_ms as u128) } else { 0 };
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
            ((x as f32) + offset.x()) * self.pixel_delta_u +
            ((y as f32) + offset.y()) * self.pixel_delta_v;
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_center - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.center + p.x() * self.defocus_disk_u + p.y() * self.defocus_disk_v
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_f32() - 0.5, random_f32() - 0.5, 0.0)
    }
}
