#![no_std]

pub mod cpu;
pub mod memory;
pub mod opcodes;

pub use self::cpu::Cpu;
pub use self::memory::Memory;

pub trait Video {
    fn is_running(&self) -> bool;
    fn render(&mut self);
}
