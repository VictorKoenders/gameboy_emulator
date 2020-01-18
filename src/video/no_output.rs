use gameboy_emulator::*;

pub struct NoOutput;

impl Video for NoOutput {
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
    fn render(&mut self) {}
    fn set_tile_pixel(
        &mut self,
        tile_index: u16,
        row_index: u16,
        pixel_index: usize,
        color: Color,
    ) {
    }
}
