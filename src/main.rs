extern crate gameboy_emulator;

mod video;

use gameboy_emulator::{cpu::Cpu, memory::*, Video};
use std::time::{Duration, Instant};
use structopt::StructOpt;

const TARGET_FPS: u32 = 60;

#[derive(Debug, StructOpt)]
#[structopt(name = "Gameboy emulator", about = "Gameboy emulator written in rust")]
pub struct Opts {
    /// If present, use a terminal output instead of a window
    #[structopt(short = "t", long = "terminal")]
    terminal: bool,

    /// If present, does not do any rendering. Useful for debugging the opcodes
    #[structopt(long = "no_output")]
    no_output: bool,

    /// The gameboy (.gb) rom that you want to play
    #[structopt(parse(from_os_str))]
    rom: std::path::PathBuf,
}

fn main() {
    use std::io::Read;
    let opts = Opts::from_args();

    let mut video: Box<dyn Video> = if opts.no_output {
        Box::new(video::NoOutput)
    } else if opts.terminal {
        Box::new(video::TerminalVideo::init())
    } else {
        Box::new(video::MinifbVideo::init())
    };

    let mut fs = std::fs::File::open(opts.rom).expect("Could not open file");
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

    let mut fixed = [0u8; CARTRIDGE_ROM_FIXED_BANK_SIZE];
    fixed.copy_from_slice(&rom[..CARTRIDGE_ROM_FIXED_BANK_SIZE]);

    let switchable_banks_len =
        (rom.len() - CARTRIDGE_ROM_FIXED_BANK_SIZE) / CARTRIDGE_ROM_SWITCHABLE_BANK_SIZE;
    let mut switchable_roms = Vec::with_capacity(switchable_banks_len);

    for i in 0..switchable_banks_len {
        let mut bank = [0u8; CARTRIDGE_ROM_SWITCHABLE_BANK_SIZE];
        let start = CARTRIDGE_ROM_FIXED_BANK_SIZE + CARTRIDGE_ROM_SWITCHABLE_BANK_SIZE * i;
        let end = (start + CARTRIDGE_ROM_SWITCHABLE_BANK_SIZE).min(rom.len());
        let len = end - start;
        bank[..len].copy_from_slice(&rom[start..end]);

        switchable_roms.push(bank);
    }
    println!(
        "Loaded 1 fixed and {} switchable banks (total {} bytes)",
        switchable_banks_len,
        rom.len()
    );

    let mut memory = Memory::new(fixed, &switchable_roms, &mut *video);
    let mut cpu = Cpu::default();

    let mut last_frame_start = Instant::now();
    let target_frame_time = Duration::from_millis(1000 / TARGET_FPS as u64);

    memory.video.render();

    while memory.video.is_running() {
        gameboy_emulator::opcodes::execute(&mut memory, &mut cpu);

        memory.update_scanline(&mut cpu.scanline_cycles);

        if cpu.frame_elapsed(TARGET_FPS) {
            memory.video.render();

            let diff = Instant::now().duration_since(last_frame_start);
            if target_frame_time > diff && !opts.no_output {
                let sleep_time = target_frame_time - diff;
                std::thread::sleep(sleep_time);
            }
            last_frame_start = Instant::now();
        }
    }
}
