#![allow(dead_code)]

use crate::Video;
use core::ops::RangeInclusive;

pub const INTERRUPT_ADDRESS: u16 = 0xFFFF;
pub const BIOS: [u8; 256] = [
    0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
    0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
    0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
    0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
    0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
    0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
    0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
    0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
    0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xF2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
    0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
    0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
    0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
    0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3c, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x4C,
    0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
    0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50,
];

struct MemMap([u8; 0x10_000]);

impl MemMap {
    pub fn new(
        switched_bank: &[u8; CARTRIDGE_ROM_SWITCHABLE_BANK_SIZE],
        fixed_bank: &[u8; CARTRIDGE_ROM_FIXED_BANK_SIZE],
    ) -> Self {
        let mut mem = MemMap([0u8; 0x10_000]);
        mem.0[CARTRIDGE_ROM_SWITCHABLE].copy_from_slice(switched_bank);
        mem.0[..CARTRIDGE_ROM_FIXED_BANK_SIZE].copy_from_slice(fixed_bank);
        mem
    }
}

/// $FFFF Interrupt Enable Flag
const INTERRUPT_ENABLE_FLAG: usize = 0xFFFF;
/// $FF80-$FFFE Zero Page - 127 bytes
const ZERO_PAGE: RangeInclusive<usize> = 0xFF80..=0xFFFE;
/// $FF00-$FF7F Hardware I/O Registers
const HARDWARE_IO_REGISTERS: RangeInclusive<usize> = 0xFF00..=0xFF7F;
/// $FEA0-$FEFF Unusable Memory
const UNUSABLE_MEMORY: RangeInclusive<usize> = 0xFEA0..=0xFEFF;
/// $FE00-$FE9F OAM - Object Attribute Memory
const OBJECT_ATTRIBUTE_MEMORY: RangeInclusive<usize> = 0xFE00..=0xFE9F;
/// $E000-$FDFF Echo RAM - Reserved, Do Not Use
const ECHO_RAM: RangeInclusive<usize> = 0xE000..=0xFDFF;
/// $D000-$DFFF Internal RAM - Bank 1-7 (switchable - CGB only)
const INTERNAL_RAM_BANKS: RangeInclusive<usize> = 0xD000..=0xDFFF;
/// $C000-$CFFF Internal RAM - Bank 0 (fixed)
const INTERNAL_RAM: RangeInclusive<usize> = 0xC000..=0xCFFF;
/// $A000-$BFFF Cartridge RAM (If Available)
const CARTRIDGE_RAM: RangeInclusive<usize> = 0xA000..=0xBFFF;
/// $9C00-$9FFF BG Map Data 2
const BG_MAP_DATA_2: RangeInclusive<usize> = 0x9C00..=0x9FFF;
/// $9800-$9BFF BG Map Data 1
const BG_MAP_DATA_1: RangeInclusive<usize> = 0x9800..=0x9BFF;
/// $8000-$97FF Character RAM
const CHARACTER_RAM: RangeInclusive<usize> = 0x4000..=0x7FFF;
/// $4000-$7FFF Cartridge ROM - Switchable Banks 1-xx
const CARTRIDGE_ROM_SWITCHABLE: RangeInclusive<usize> = 0x4000..=0x7FFF;
/// $0150-$3FFF Cartridge ROM - Bank 0 (fixed)
const CARTRIDGE_ROM_FIXED: RangeInclusive<usize> = 0x0150..=0x3FFF;
/// $0100-$014F Cartridge Header Area
const CARTRIDGE_HEADER_AREA: RangeInclusive<usize> = 0x0100..=0x014F;
/// $0000-$00FF Restart and Interrupt Vectors
const RESTART_AND_INTERRUPT_VECTORS: RangeInclusive<usize> = 0x0000..=0x00FF;
/// $0000-$00FF BIOS_AREA
const BIOS_AREA: RangeInclusive<usize> = 0x0000..=0x00FF;

pub const CARTRIDGE_ROM_FIXED_BANK_SIZE: usize = 0x4000;
const CARTRIDGE_ROM_SWITCHABLE_BANK_SIZE: usize = 0x4000;
#[test]
fn mem_size_sanity_check() {
    assert_eq!(BIOS.len(), BIOS_AREA.end() - BIOS_AREA.start() + 1);
    assert_eq!(
        CARTRIDGE_ROM_SWITCHABLE_BANK_SIZE,
        CARTRIDGE_ROM_SWITCHABLE.end() - CARTRIDGE_ROM_SWITCHABLE.start() + 1
    );
}
pub struct Memory<'a> {
    map: MemMap,
    switchable_banks: &'a [[u8; CARTRIDGE_ROM_SWITCHABLE_BANK_SIZE]],
    bios_loaded: bool,
    pub video: &'a mut dyn Video,
}

impl<'a> Memory<'a> {
    pub fn new(
        fixed_bank: [u8; CARTRIDGE_ROM_FIXED_BANK_SIZE],
        switchable_banks: &'a [[u8; CARTRIDGE_ROM_SWITCHABLE_BANK_SIZE]],
        video: &'a mut dyn Video,
    ) -> Self {
        let switched_bank = if switchable_banks.is_empty() {
            &[0u8; CARTRIDGE_ROM_SWITCHABLE_BANK_SIZE]
        } else {
            &switchable_banks[0]
        };
        Memory {
            map: MemMap::new(switched_bank, &fixed_bank),
            bios_loaded: true,
            video,
            switchable_banks,
        }
    }

    pub fn disable_bios(&mut self) {
        self.bios_loaded = false;
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        if self.bios_loaded && address < 0x0100 {
            BIOS[address as usize]
        } else {
            self.map.0[address as usize]
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let high = self.read_byte(address);
        let low = self.read_byte(address + 1);
        bytes_to_word(high, low)
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        if self.bios_loaded && address < 0x0100 {
            unimplemented!()
        } else {
            self.map.0[address as usize] = value;
        }

        if is_in_vram(address) {
            todo!("Writing to vram 0x{:04X} (val {})", address, value);
        }
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let (high, low) = word_to_bytes(value);
        self.write_byte(address, high);
        self.write_byte(address + 1, low);
    }
}

fn is_in_vram(address: u16) -> bool {
    address >= 0x8000 && address <= 0x9FFF
}

const fn bytes_to_word(high: u8, low: u8) -> u16 {
    (low as u16) << 8 | (high as u16)
}

const fn word_to_bytes(word: u16) -> (u8, u8) {
    let high = (word >> 8) as u8;
    let low = word as u8;
    (low, high)
}
