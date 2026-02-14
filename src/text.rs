use font8x8::legacy::BASIC_LEGACY;

use crate::vec3::Point3; // U+0000 - U+007F

pub fn display_char_at(buffer: Vec<u32>, coord: Point3, char: char) {
    let buffer_size = buffer.len();
    for i in 0..8 {
        for j in 0..8 {
            let pixel_col = buffer.get(0);
        }
    }
}
