use crate::Video;
use minifb::*;

pub struct MinifbVideo {
    window: Window,
    buffer: Vec<u32>,
}

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

impl MinifbVideo {
    pub fn init() -> Self {
        let window = Window::new("Gameboy", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
        let buffer = vec![from_u8_rgb(0, 0, 0); WIDTH * HEIGHT];
        Self { window, buffer }
    }
}

impl Video for MinifbVideo {
    fn is_running(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    fn render(&mut self) {
        self.window
            .update_with_buffer_size(&self.buffer, WIDTH, HEIGHT)
            .expect("Could not draw");
    }
}
