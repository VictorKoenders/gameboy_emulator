#![allow(dead_code)]

use crate::{Color, Video};
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
const VIDEO_RAM: RangeInclusive<usize> = 0x8000..=0x9FFF;
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
pub const CARTRIDGE_ROM_SWITCHABLE_BANK_SIZE: usize = 0x4000;

const REGISTER_CHANNEL_ONE_SOUND_LENGTH_WAVE_PATTERN: u16 = 0xFF11;
const REGISTER_CHANNEL_ONE_VOLUME_ENVELOPE: u16 = 0xFF12;
const REGISTER_CHANNEL_CONTROL: u16 = 0xFF24;
const REGISTER_SOUND_SELECTION: u16 = 0xFF25;
const REGISTER_SOUND_ENABLE: u16 = 0xFF26;
const REGISTER_LCD_CONTROL: u16 = 0xFF40;
const REGISTER_SCROLL_POSITION_Y: u16 = 0xFF42;
const REGISTER_SCANLINE_Y: u16 = 0xFF44;
const REGISTER_BACKGROUND_PALETTE: u16 = 0xFF47;

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
    scanline: ScanLine,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum ScanLine {
    Oam,
    Vram,
    HorizontalBlank,
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
            scanline: ScanLine::Oam,
        }
    }

    pub fn update_scanline(&mut self, scanline_counter: &mut u16) {
        match self.scanline {
            ScanLine::Oam => {
                if *scanline_counter >= 80 {
                    self.scanline = ScanLine::Vram;
                    *scanline_counter -= 80;
                }
            }
            ScanLine::Vram => {
                if *scanline_counter >= 172 {
                    self.scanline = ScanLine::HorizontalBlank;
                    *scanline_counter -= 172;
                }
            }
            ScanLine::HorizontalBlank => {
                if *scanline_counter >= 204 {
                    self.scanline = ScanLine::Oam;
                    self.increment_scanline_y();
                    *scanline_counter -= 204;
                }
            }
        }
    }

    fn increment_scanline_y(&mut self) {
        let y = &mut self.map.0[0xff44];
        *y += 1;
        if *y == 154 {
            *y = 0;
        }
    }

    pub fn disable_bios(&mut self) {
        self.bios_loaded = false;
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        if self.bios_loaded && address < 0x0100 {
            BIOS[address as usize]
        } else {
            let val = self.map.0[address as usize];

            if HARDWARE_IO_REGISTERS.contains(&(address as usize)) {
                match address {
                    REGISTER_SCROLL_POSITION_Y => {} // Read scroll Y
                    REGISTER_SCANLINE_Y => {}        // Read vertical scanline
                    _ => todo!(
                        "Reading from hardware register 0x{:04x} (value 0x{:02X})",
                        address,
                        val
                    ),
                }
            }

            val
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        if self.bios_loaded && address < 0x0100 {
            unimplemented!()
        } else {
            self.map.0[address as usize] = value;
        }

        if HARDWARE_IO_REGISTERS.contains(&(address as usize)) {
            match address {
                REGISTER_CHANNEL_ONE_SOUND_LENGTH_WAVE_PATTERN => {
                    println!("Channel 1 {:?}", ChannelOneSoundLengthWavePattern(value))
                }
                REGISTER_CHANNEL_ONE_VOLUME_ENVELOPE => {
                    println!("Channel 1 {:?}", ChannelOneVolumeEnvelope(value))
                }
                REGISTER_CHANNEL_CONTROL => println!("Channel control {:?}", ChannelControl(value)),
                REGISTER_SOUND_SELECTION => println!("Sound selection {:?}", SoundSelection(value)),
                REGISTER_SOUND_ENABLE => println!("Sound {:?}", SoundEnable(value)),
                REGISTER_LCD_CONTROL => println!("{:?}", LcdControl(value)),
                REGISTER_SCROLL_POSITION_Y => println!("Writing scroll position y {:?}", value),
                REGISTER_BACKGROUND_PALETTE => {
                    println!("Background palette {:?}", BackgroundPalette(value))
                }
                _ => todo!(
                    "Writing to hardware register 0x{:04X} (value 0x{:02X})",
                    address,
                    value
                ),
            }
        }

        if VIDEO_RAM.contains(&(address as usize)) {
            // More info: https://blog.ryanlevick.com/DMG-01/public/book/graphics/tile_ram.html
            let normalized_index = (address & 0xFFFE) as usize;
            // First we need to get the two bytes that encode the tile row.
            let byte1 = self.map.0[normalized_index];
            let byte2 = self.map.0[normalized_index + 1];

            // A tiles is 8 rows tall. Since each row is encoded with two bytes a tile
            // is therefore 16 bytes in total.
            let tile_index = (address - (*VIDEO_RAM.start() as u16)) / 16;
            // Every two bytes is a new row
            let row_index = (address % 16) / 2;

            // Now we're going to loop 8 times to get the 8 pixels that make up a given row.
            for pixel_index in 0..8 {
                // To determine a pixel's value we must first find the corresponding bit that encodes
                // that pixels value:
                // 1111_1111
                // 0123 4567
                //
                // As you can see the bit that corresponds to the nth pixel is the bit in the nth
                // position *from the left*. Bits are normally indexed from the right.
                //
                // To find the first pixel (a.k.a pixel 0) we find the left most bit (a.k.a bit 7). For
                // the second pixel (a.k.a pixel 1) we first the second most left bit (a.k.a bit 6) and
                // so on.
                //
                // We then create a mask with a 1 at that position and 0s everywhere else.
                //
                // Bitwise ANDing this mask with our bytes will leave that particular bit with its
                // original value and every other bit with a 0.
                let mask = 1 << (7 - pixel_index);
                let lsb = byte1 & mask > 0;
                let msb = byte2 & mask > 0;
                self.video
                    .set_tile_pixel(tile_index, row_index, pixel_index, (lsb, msb).into());
            }
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let high = self.read_byte(address);
        let low = self.read_byte(address + 1);
        bytes_to_word(high, low)
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let (high, low) = word_to_bytes(value);
        self.write_byte(address, high);
        self.write_byte(address + 1, low);
    }
}

struct ChannelOneSoundLengthWavePattern(u8);

impl core::fmt::Debug for ChannelOneSoundLengthWavePattern {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        let duty = (self.0 & 0b1100_0000) >> 6;
        let length_wave = self.0 & 0b0011_1111;
        write!(
            fmt,
            "Wave duty: {}, sound length: {} seconds",
            match duty {
                0b00 => "12.5%",
                0b01 => "25%",
                0b10 => "50%",
                0b11 => "75%",
                _ => unreachable!(),
            },
            1.0 / ((64.0 - length_wave as f32) * 256.0),
        )
    }
}

struct ChannelOneVolumeEnvelope(u8);

impl core::fmt::Debug for ChannelOneVolumeEnvelope {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        let initial_volume = (self.0 & 0b0111_0000) >> 4;
        let envelope_direction = (self.0 & 0b0000_1000) >> 3;
        let numer_of_envelope_sweeps = self.0 & 0b0000_0111;

        write!(
            fmt,
            "Initial volume: {}, direction: {} ({}), number of sweeps: {}",
            initial_volume,
            envelope_direction,
            if envelope_direction > 0 {
                "increase"
            } else {
                "decrease"
            },
            numer_of_envelope_sweeps
        )
    }
}

struct SoundEnable(u8);
impl core::fmt::Debug for SoundEnable {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        let all_sound_off = (self.0 & 0b1000_0000) > 0;
        let sound_4_on = (self.0 & 0b0000_1000) > 0;
        let sound_3_on = (self.0 & 0b0000_0100) > 0;
        let sound_2_on = (self.0 & 0b0000_0010) > 0;
        let sound_1_on = (self.0 & 0b0000_0001) > 0;

        write!(
            fmt,
            "Sound enable: {} (4: {}, 3: {}, 2: {}, 1: {})",
            !all_sound_off, sound_4_on, sound_3_on, sound_2_on, sound_1_on
        )
    }
}

struct SoundSelection(u8);
impl core::fmt::Debug for SoundSelection {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        let masks = &[
            (0b1000_0000, "channel 4 to SO2"),
            (0b0100_0000, "channel 3 to SO2"),
            (0b0010_0000, "channel 2 to SO2"),
            (0b0001_0000, "channel 1 to SO2"),
            (0b0000_1000, "channel 4 to SO1"),
            (0b0000_0100, "channel 3 to SO1"),
            (0b0000_0010, "channel 2 to SO1"),
            (0b0000_0001, "channel 1 to SO1"),
        ];
        let mut first = true;
        for (mask, text) in masks {
            if self.0 & mask > 0 {
                if first {
                    first = false;
                } else {
                    write!(fmt, ", ")?;
                }
                write!(fmt, "{}", text)?;
            }
        }
        Ok(())
    }
}

struct ChannelControl(u8);
impl core::fmt::Debug for ChannelControl {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        let so2_enable = (self.0 & 0b1000_0000) > 0;
        let so2_volume = (self.0 & 0b0111_0000) >> 4;
        let so1_enable = (self.0 & 0b0000_1000) > 0;
        let so1_volume = self.0 & 0b0000_0111;

        if so2_enable {
            write!(fmt, "so2: {}", so2_volume)?;
        } else {
            write!(fmt, "so2: disabled")?;
        }
        write!(fmt, ", ")?;
        if so1_enable {
            write!(fmt, "so1: {}", so1_volume)
        } else {
            write!(fmt, "so1: disabled")
        }
    }
}

struct BackgroundPalette(u8);
impl core::fmt::Debug for BackgroundPalette {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        let shade_3: Color = ((self.0 & 0b1100_0000) >> 6).into();
        let shade_2: Color = ((self.0 & 0b0011_0000) >> 4).into();
        let shade_1: Color = ((self.0 & 0b0000_1100) >> 2).into();
        let shade_0: Color = (self.0 & 0b0000_0011).into();

        write!(
            fmt,
            "shade 3: {:?}, shade 2: {:?}, shade 1: {:?}, shade 0: {:?}",
            shade_3, shade_2, shade_1, shade_0
        )
    }
}

struct LcdControl(u8);
impl core::fmt::Debug for LcdControl {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "LCD control {:08b}", self.0)
    }
}

const fn bytes_to_word(high: u8, low: u8) -> u16 {
    (low as u16) << 8 | (high as u16)
}

const fn word_to_bytes(word: u16) -> (u8, u8) {
    let high = (word >> 8) as u8;
    let low = word as u8;
    (low, high)
}
