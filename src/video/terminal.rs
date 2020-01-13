use crate::Video;
use drawille::*;
use terminal_size::*;

pub struct TerminalVideo {
    canvas: Canvas,
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
        Self { canvas }
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
}
