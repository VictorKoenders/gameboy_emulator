use super::Tile;
use gameboy_emulator::{Color, Video};
use minifb::*;

pub struct MinifbVideo {
    window: Window,
    buffer: Vec<u32>,
    tiles: [Tile; 384],
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
        MinifbVideo {
            window,
            buffer,
            tiles: [Tile::default(); 384],
        }
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

    fn set_tile_pixel(
        &mut self,
        tile_index: u16,
        row_index: u16,
        pixel_index: usize,
        color: Color,
    ) {
        self.tiles[tile_index as usize].set(row_index as usize, pixel_index, color);
    }
}
