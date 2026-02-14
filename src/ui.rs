extern crate font8x8;
pub mod text;
use crate::{ Coord2, WindowBuffer };

use crate::ui::text::TextString;
pub struct Ui {
    pub scale: u16,
    pub lines_content: Vec<TextString>,
    pub inter_lines_height: u16,
}

impl Ui {
    pub fn display_at(&self, buffer: &mut WindowBuffer, coord: Coord2) {
        let mut current_coord = coord;
        for line in &self.lines_content {
            line.display_at(buffer, self.scale, current_coord);
            let offset = Coord2 {
                x: 0,
                y: (self.scale as usize) *
                (8 * (line.font_size as usize) + (self.inter_lines_height as usize)),
            };
            current_coord = current_coord + offset;
        }
    }
}
