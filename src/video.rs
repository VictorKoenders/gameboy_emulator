use minifb::*;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

pub struct Video {
    window: Window,
    buffer: Vec<u32>,
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

impl Video {
    pub fn init() -> Video {
        let window = Window::new("Gameboy", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
        let buffer = vec![from_u8_rgb(0, 0, 0); WIDTH * HEIGHT];
        Video { window, buffer }
    }

    pub fn is_running(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn render(&mut self) {
        self.window
            .update_with_buffer_size(&self.buffer, WIDTH, HEIGHT)
            .expect("Could not draw");
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    Black = 0b00,
    LightGray = 0b01,
    DarkGray = 0b10,
    White = 0b11,
}

