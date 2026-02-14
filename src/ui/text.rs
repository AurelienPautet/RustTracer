use font8x8::{ BASIC_FONTS, UnicodeFonts };

use crate::{ Coord2, WindowBuffer, vec3::Color };

pub struct TextString {
    pub content: String,
    pub font_size: u16,
    pub color: Color,
    pub opacity: f32,
}

impl TextString {
    pub fn display_at(&self, buffer: &mut WindowBuffer, ui_scale: u16, coord: Coord2) {
        let char_size = self.font_size * ui_scale;
        let mut i = 0;
        for c in self.content.chars() {
            let decalage = Coord2 {
                x: (i as usize) * 8 * (char_size as usize),
                y: 0,
            };
            display_char_at(buffer, char_size, coord + decalage, self.color, self.opacity, c);
            i = i + 1;
        }
    }
}

fn display_char_at(
    buffer: &mut WindowBuffer,
    char_size: u16,
    coord: Coord2,
    color: Color,
    opacity: f32,
    c: char
) {
    let _buffer_size = buffer.size.area();
    let mut y = 0;
    if let Some(glyph) = BASIC_FONTS.get(c) {
        for g in &glyph {
            for x in 0..8 {
                match *g & (1 << x) {
                    0 => (),
                    _ => {
                        for rx in 0..char_size {
                            for ry in 0..char_size {
                                let decalage = Coord2 {
                                    x: (x as usize) * (char_size as usize) + (rx as usize),
                                    y: (y as usize) * (char_size as usize) + (ry as usize),
                                };

                                let index = buffer.get_index(coord + decalage);
                                let Some(u32_color) = buffer.content.get(index) else {
                                    return;
                                };
                                let background_col = Color::from_u32(*u32_color);
                                let pixel_col = background_col * (1.0 - opacity) + color * opacity;
                                buffer.content[index] = pixel_col.to_u32();
                            }
                        }
                    }
                }
            }
            y = y + 1;
        }
    }
}
