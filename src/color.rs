use crate::{ interval::Interval, vec3::Vec3 };

pub fn write_color(color: Vec3) {
    let intensity = Interval::new(0.0, 0.999);
    let ir: u8 = (255.99 * intensity.clamp(color.x())) as u8;
    let ig: u8 = (255.99 * intensity.clamp(color.y())) as u8;
    let ib: u8 = (255.99 * intensity.clamp(color.z())) as u8;

    println!("{} {} {}", ir, ig, ib);
}
