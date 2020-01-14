// #![no_std]

pub mod cpu;
pub mod memory;
pub mod opcodes;

pub use self::{cpu::Cpu, memory::Memory};

pub trait Video {
    fn is_running(&self) -> bool;
    fn render(&mut self);
    fn set_tile_pixel(&mut self, tile_index: u16, row_index: u16, pixel_index: usize, color: Color);
    fn button_state(&mut self) -> ButtonState;
    fn direction_state(&mut self) -> DirectionState;
}

#[derive(Debug)]
pub struct ButtonState {
    pub start: bool,
    pub select: bool,
    pub a: bool,
    pub b: bool,
}

#[derive(Debug)]
pub struct DirectionState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
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

impl Color {
    pub fn to_u8_rgb(self) -> u32 {
        let (r, g, b) = match self {
            Color::Black => (0, 0, 0),
            Color::LightGray => (170, 170, 170),
            Color::DarkGray => (85, 85, 85),
            Color::White => (255, 255, 255),
        };
        (r << 16) | (g << 8) | b
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

impl From<u8> for Color {
    fn from(b: u8) -> Self {
        match b {
            0b00 => Color::White,
            0b01 => Color::LightGray,
            0b10 => Color::DarkGray,
            0b11 => Color::Black,
            _ => unimplemented!(),
        }
    }
}
