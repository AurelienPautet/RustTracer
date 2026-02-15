use std::fs::OpenOptions;
use std::io::prelude::*;
use rfd::FileDialog;
use crate::camera::Camera;

pub fn save_image(camera: &Camera) {
    println!("Saving file to with name ");
    let files = FileDialog::new().set_file_name("render.ppm").save_file();

    match files {
        None => println!("The user cancelled the save."),
        Some(path_buf) => {
            let mut file = match
                OpenOptions::new().write(true).create(true).truncate(true).open(path_buf)
            {
                Ok(f) => f,
                Err(_) => {
                    return;
                }
            };

            writeln!(file, "P3").expect("write failed");
            writeln!(file, "{} {}", camera.image_size.w, camera.image_size.h).expect(
                "write failed"
            );
            writeln!(file, "255").expect("write failed");

            let inv_samples = if camera.sample_current > 0 {
                1.0 / (camera.sample_current as f32)
            } else {
                1.0
            };

            for j in 0..camera.image_size.h {
                for i in 0..camera.image_size.w {
                    let pixel_color =
                        camera.color_buffer[j * camera.image_size.w + i] * inv_samples;
                    let r = pixel_color.x().sqrt();
                    let g = pixel_color.y().sqrt();
                    let b = pixel_color.z().sqrt();
                    let ir = (255.999 * r) as u8;
                    let ig = (255.999 * g) as u8;
                    let ib = (255.999 * b) as u8;
                    writeln!(file, "{} {} {}", ir, ig, ib).expect("write failed");
                }
            }
        }
    }
}
