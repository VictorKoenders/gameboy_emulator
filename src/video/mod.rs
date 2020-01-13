mod minifb;
mod terminal;

pub use self::minifb::MinifbVideo;
pub use self::terminal::TerminalVideo;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    Black = 0b00,
    LightGray = 0b01,
    DarkGray = 0b10,
    White = 0b11,
}
