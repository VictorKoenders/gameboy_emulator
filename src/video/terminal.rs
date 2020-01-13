use super::Tile;
use drawille::*;
use gameboy_emulator::{Color, Video};
use terminal_size::*;

pub struct TerminalVideo {
    canvas: Canvas,
    tiles: [Tile; 384],
}

const HEIGHT: u32 = 144;
const WIDTH: u32 = 160;

const HEIGHT_CHARACTERS: u32 = HEIGHT / 4;
const WIDTH_CHARACTERS: u32 = WIDTH / 2;

impl TerminalVideo {
    pub fn init() -> Self {
        if let Some((Width(w), Height(h))) = terminal_size() {
            println!("Terminal is {}x{}", w, h);
            if (w as u32) < WIDTH_CHARACTERS || (h as u32) < HEIGHT_CHARACTERS {
                panic!(
                    "Terminal should be at least {}x{} (currently {}x{})",
                    WIDTH_CHARACTERS, HEIGHT_CHARACTERS, w, h
                );
            }
        } else {
            panic!("Could not query terminal size");
        }
        let mut canvas = Canvas::new(160, 144);
        canvas.line(10, 10, 40, 40);
        Self {
            canvas,
            tiles: [Tile::default(); 384],
        }
    }
}

impl Video for TerminalVideo {
    fn is_running(&self) -> bool {
        true
    }
    fn render(&mut self) {
        print!("\x1B[2J"); // clear screen
        println!("{}", self.canvas.frame());
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
