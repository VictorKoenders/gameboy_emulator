#![no_std]

pub mod cpu;
pub mod memory;
pub mod opcodes;

pub use self::cpu::Cpu;
pub use self::memory::Memory;

pub trait Video {
    fn is_running(&self) -> bool;
    fn render(&mut self);
    fn set_tile_pixel(&mut self, tile_index: u16, row_index: u16, pixel_index: usize, color: Color);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    Black = 0b00,
    LightGray = 0b01,
    DarkGray = 0b10,
    White = 0b11,
}

impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}

impl From<(bool, bool)> for Color {
    fn from((lsb, msb): (bool, bool)) -> Self {
        match (lsb, msb) {
            (true, true) => Color::White,
            (true, false) => Color::DarkGray,
            (false, true) => Color::LightGray,
            (false, false) => Color::Black,
        }
    }
}
