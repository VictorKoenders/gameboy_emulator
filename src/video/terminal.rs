use super::Tile;
use drawille::*;
use gameboy_emulator::*;
use std::io::{stdout, Stdout, Write};
use termion::terminal_size;

pub struct TerminalVideo {
    canvas: Canvas,
    tiles: [Tile; 384],
    stdout: Stdout,
}

const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;

const HEIGHT_CHARACTERS: u32 = HEIGHT / 4;
const WIDTH_CHARACTERS: u32 = WIDTH / 2;

impl TerminalVideo {
    pub fn init() -> Self {
        let (w, h) = terminal_size().expect("Could not query terminal size");
        println!("Terminal is {}x{}", w, h);
        if (w as u32) < WIDTH_CHARACTERS || (h as u32) < HEIGHT_CHARACTERS {
            panic!(
                "Terminal should be at least {}x{} (currently {}x{})",
                WIDTH_CHARACTERS, HEIGHT_CHARACTERS, w, h
            );
        }
        let mut canvas = Canvas::new(WIDTH, HEIGHT);
        canvas.line(0, 0, WIDTH, 0);
        canvas.line(0, 0, 0, HEIGHT);
        canvas.line(WIDTH, 0, WIDTH, HEIGHT);
        canvas.line(0, HEIGHT, WIDTH, HEIGHT);

        Self {
            canvas,
            tiles: [Tile::default(); 384],
            stdout: stdout(),
        }
    }
}

impl Video for TerminalVideo {
    fn button_state(&mut self) -> ButtonState {
        ButtonState {
            a: false,
            b: false,
            select: false,
            start: false,
        }
    }
    fn direction_state(&mut self) -> DirectionState {
        DirectionState {
            left: false,
            right: false,
            up: false,
            down: false,
        }
    }

    fn is_running(&self) -> bool {
        true
    }
    fn render(&mut self) {
        writeln!(
            self.stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            self.canvas.frame()
        )
        .unwrap();
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
