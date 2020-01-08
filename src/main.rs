mod cpu;
mod memory;
mod opcodes;
mod video;

use std::time::{Duration, Instant};

pub mod utils {
    pub const fn bytes_to_word(high: u8, low: u8) -> u16 {
        (low as u16) << 8 | (high as u16)
    }

    pub const fn word_to_bytes(word: u16) -> (u8, u8) {
        let high = (word >> 8) as u8;
        let low = word as u8;
        (low, high)
    }
}

const TARGET_FPS: u64 = 30;

fn main() {
    let file = std::env::args()
        .nth(1)
        .expect("Missing argument: <file>.gb");

    let mut video = video::Video::init();
    let mut memory = memory::Memory::load_from_file(&mut video, &file);
    let mut cpu = cpu::Cpu::default();

    let mut last_frame_start = Instant::now();
    let target_frame_time = Duration::from_millis(1000 / TARGET_FPS);

    println!("Update frame");
    memory.video.render();

    while memory.video.is_running() {
        opcodes::execute(&mut memory, &mut cpu);

        if cpu.frame_elapsed(TARGET_FPS) {
            println!("Update frame");
            memory.video.render();

            let diff = Instant::now().duration_since(last_frame_start);
            if target_frame_time > diff {
                let sleep_time = target_frame_time - diff;
                std::thread::sleep(sleep_time);
            }
            last_frame_start = Instant::now();
        }
    }
}
