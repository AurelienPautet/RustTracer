pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod interval;
pub mod camera;
pub mod material;
pub mod text;
pub mod scene;
pub mod ui;

use crate::camera::Direction;
use crate::vec3::{ Color, Point3 };
pub use std::f32::{ INFINITY, NEG_INFINITY, consts::PI };
use std::time::Instant;
use crate::scene::Scene;
use minifb::{ Key, Window, WindowOptions };

fn _degrees_to_radians(degrees: f32) -> f32 {
    (degrees * PI) / 180.0
}

pub fn random_f32() -> f32 {
    rand::random()
}

pub fn random_f32_range(min: f32, max: f32) -> f32 {
    rand::random_range(min..=max)
}
#[derive(Debug, Clone, Copy)]
pub struct Size {
    w: usize,
    h: usize,
}
impl Size {
    pub fn area(&self) -> usize {
        self.h * self.w
    }
}

impl PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w && self.h == other.h
    }
}

pub struct WindowBuffer {
    size: Size,
    content: Vec<u32>,
}

impl WindowBuffer {
    fn resize(&mut self, new_size: Size) {
        self.size = new_size;
        self.content.resize(self.size.area(), 0);
    }
}

fn main() {
    let mut size = Size {
        w: 400,
        h: 400,
    };
    let mut scenes = vec![
        Scene::create_scene1(size),
        Scene::create_scene2(size),
        Scene::create_scene3(size)
    ];
    let scenes_len = scenes.len();
    let mut current_scene_idx = 0;

    let mut window = Window::new("RustTracer", size.w, size.h, WindowOptions {
        resize: true,
        ..WindowOptions::default()
    }).unwrap();
    let mut last_mouse_pos: (f32, f32) = (0.0, 0.0);
    let mut first_mouse = true;

    window.set_target_fps(200);
    let mut window_buffer = WindowBuffer { size: size, content: vec![0; size.area()] };
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start: Instant = Instant::now();

        let (new_w, new_h) = window.get_size();
        let new_size = Size { h: new_h, w: new_w };
        if size != new_size {
            println!("resize");
            size = new_size;
            for scene in &mut scenes {
                scene.camera.resize(size);
            }
            window_buffer.resize(new_size);
            dbg!(window_buffer.size);
        }

        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(minifb::MouseMode::Pass) {
            if first_mouse {
                last_mouse_pos = (mouse_x, mouse_y);
                first_mouse = false;
            }

            let x_offset = mouse_x - last_mouse_pos.0;
            let y_offset = last_mouse_pos.1 - mouse_y;
            last_mouse_pos = (mouse_x, mouse_y);
            let min = 0.0001;
            if window.get_mouse_down(minifb::MouseButton::Left) {
                if x_offset.abs() > min || y_offset.abs() > min {
                    scenes[current_scene_idx].camera.rotate_camera(x_offset, y_offset);
                }
            }
        }
        let scene = &mut scenes[current_scene_idx];
        if window.is_key_down(Key::W) {
            scene.camera.move_camera(Direction::Forward);
        }
        if window.is_key_down(Key::S) {
            scene.camera.move_camera(Direction::Backward);
        }
        if window.is_key_down(Key::A) {
            scene.camera.move_camera(Direction::Left);
        }
        if window.is_key_down(Key::D) {
            scene.camera.move_camera(Direction::Right);
        }
        if window.is_key_down(Key::F) {
            scene.camera.fov = scene.camera.fov + 1.0;
            scene.camera.clear();
        }
        if window.is_key_down(Key::V) {
            scene.camera.fov = scene.camera.fov - 1.0;
            scene.camera.clear();
        }
        if window.is_key_down(Key::G) {
            scene.camera.focus_dist = scene.camera.focus_dist + 0.1;
            scene.camera.clear();
        }
        if window.is_key_down(Key::B) {
            scene.camera.focus_dist = scene.camera.focus_dist - 0.1;
            scene.camera.clear();
        }
        if window.is_key_down(Key::H) {
            scene.camera.defocus_angle = scene.camera.defocus_angle - 0.1;
            scene.camera.clear();
        }
        if window.is_key_down(Key::N) {
            scene.camera.defocus_angle = scene.camera.defocus_angle + 0.1;
            scene.camera.clear();
        }
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            current_scene_idx = (current_scene_idx + 1) % scenes_len;
        }

        scene.camera.render(&scene.world, &mut window_buffer.content);

        let elapsed_ms = start.elapsed().as_millis();
        let fps = if elapsed_ms > 0 { 1000 / (elapsed_ms as u128) } else { 0 };
        let title =
            String::from("RustTracer, fps :") +
            &fps.to_string() +
            &String::from(" sample :") +
            &scene.camera.sample_current.to_string();
        window.set_title(&title);
        window.update_with_buffer(&window_buffer.content, new_w, new_h).unwrap();
    }
}
