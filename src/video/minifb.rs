use gameboy_emulator::*;
use minifb::*;

pub struct MinifbVideo {
    window: Window,
    buffer: Vec<u32>,
}

const WIDTH: usize = 32 * 8;
const HEIGHT: usize = 32 * 8;

impl MinifbVideo {
    pub fn init() -> Self {
        let window = Window::new(
            "Gameboy",
            WIDTH,
            HEIGHT,
            WindowOptions {
                scale: Scale::X4,
                ..Default::default()
            },
        )
        .unwrap();
        let buffer = vec![Color::White.to_u8_rgb(); WIDTH * HEIGHT];
        MinifbVideo { window, buffer }
    }
}

impl Video for MinifbVideo {
    fn is_running(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    fn button_state(&mut self) -> ButtonState {
        ButtonState {
            a: self.window.is_key_down(Key::Z),
            b: self.window.is_key_down(Key::X),
            start: self.window.is_key_down(Key::A),
            select: self.window.is_key_down(Key::D),
        }
    }
    fn direction_state(&mut self) -> DirectionState {
        DirectionState {
            up: self.window.is_key_down(Key::Up),
            down: self.window.is_key_down(Key::Down),
            left: self.window.is_key_down(Key::Left),
            right: self.window.is_key_down(Key::Right),
        }
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
        let x = (tile_index % 32) * 8 + row_index;
        let y = (tile_index / 32) * 8 + pixel_index as u16;

        let index = x as usize + (y as usize * WIDTH);

        self.buffer[index] = color.to_u8_rgb();
    }
}
