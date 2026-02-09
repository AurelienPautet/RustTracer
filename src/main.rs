use std::io::{ self, Write };

pub mod vec3;
pub mod color;

use crate::vec3::{ Vec3, Point3, Color };
use crate::color::write_color;

struct Image {
    width: u16,
    height: u16,
}

const IMAGE: Image = Image {
    width: 256,
    height: 256,
};

fn main() {
    println!("P3\n{} {}\n255", IMAGE.width, IMAGE.height);
    for y in 0..IMAGE.height {
        eprint!("\rScanlines remaining: {} ", IMAGE.height - 1 - y);
        io::stderr().flush().unwrap();
        for x in 0..IMAGE.width {
            let r = (x as f64) / ((IMAGE.width - 1) as f64);
            let g = (y as f64) / ((IMAGE.height - 1) as f64);
            let b: f64 = 0.0;
            write_color(Color::new(r, g, b));
        }
    }
}
