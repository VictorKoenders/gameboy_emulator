mod minifb;
mod terminal;

pub use self::{minifb::MinifbVideo, terminal::TerminalVideo};

use gameboy_emulator::Color;

#[derive(Default, Clone, Copy)]
pub struct Tile {
    color: [[Color; 8]; 8],
}

impl Tile {
    pub fn set(&mut self, row_index: usize, pixel_index: usize, color: Color) {
        self.color[row_index][pixel_index] = color;
    }
}
