mod cpu;
mod memory;
mod opcodes;
mod video;

use std::time::{Duration, Instant};
use video::MinifbVideo;
use structopt::StructOpt;

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

#[derive(Debug, StructOpt)]
#[structopt(name = "Gameboy emulator", about = "Gameboy emulator written in rust")]
pub struct Opts {
    /// If present, use a terminal output instead of a window
    #[structopt(short = "t", long = "terminal")]
    terminal: bool,

    /// The gameboy (.gb) rom that you want to play
    #[structopt(parse(from_os_str))]
    rom: std::path::PathBuf,
}

fn main() {
    let opts = Opts::from_args();

    let mut video: Box<dyn video::Video> = if opts.terminal {
        Box::new(video::TerminalVideo::init())
    } else { 
        Box::new(MinifbVideo::init())
    };

    let mut memory = memory::Memory::load_from_file(&mut *video, &opts.rom);
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
