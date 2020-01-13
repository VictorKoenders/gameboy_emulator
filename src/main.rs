extern crate gameboy_emulator;

pub use gameboy_emulator::{cpu::Cpu, memory::Memory, Video};

mod video;

use std::time::{Duration, Instant};
use structopt::StructOpt;
use video::MinifbVideo;

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

    let mut video: Box<dyn Video> = if opts.terminal {
        Box::new(video::TerminalVideo::init())
    } else {
        Box::new(MinifbVideo::init())
    };

    let mut memory = memory_from_file(&mut *video, &opts.rom);
    let mut cpu = Cpu::default();

    let mut last_frame_start = Instant::now();
    let target_frame_time = Duration::from_millis(1000 / TARGET_FPS);

    println!("Update frame");
    memory.video.render();

    while memory.video.is_running() {
        gameboy_emulator::opcodes::execute(&mut memory, &mut cpu);

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

fn memory_from_file(video: &mut dyn Video, file: impl AsRef<std::path::Path>) -> Memory {
    use std::io::Read;
    let mut fs = std::fs::File::open(file).expect("Could not open file");
    let mut rom = Vec::new();
    fs.read_to_end(&mut rom).expect("Could not read file");

    let name = match std::str::from_utf8(&rom[0x134..0x142]) {
        Ok(name) => name.trim_end_matches('\0'),
        Err(e) => {
            eprintln!("Could not load the rom name, you're probably loading an invalid cartridge");
            panic!("{:?}", e);
        }
    };
    println!("Found game: {} (0x{:X} bytes)", name, rom.len());

    let mut fixed = [0u8; gameboy_emulator::memory::CARTRIDGE_ROM_FIXED_BANK_SIZE];
    fixed.copy_from_slice(&rom[..gameboy_emulator::memory::CARTRIDGE_ROM_FIXED_BANK_SIZE]);

    Memory::new(fixed, &[], video)
}
