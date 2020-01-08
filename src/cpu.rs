use crate::memory::Memory;
use crate::utils::{bytes_to_word, word_to_bytes};

pub struct Cpu {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,

    pub flags: Flags,
    cycles: usize,
}

impl Default for Cpu {
    fn default() -> Cpu {
        Cpu {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,

            sp: 0xFFFE,
            cycles: 0,
            flags: Flags(0),
            pc: 0x0,
        }
    }
}

impl Cpu {
    pub fn program_counter(&self) -> u16 {
        self.pc
    }

    pub fn increment_program_counter(&mut self) {
        self.pc += 1;
    }

    pub fn set_program_counter(&mut self, pc: u16) {
        self.pc = pc;
    }

    pub fn clock_cycles(&mut self, cycles: usize) {
        self.cycles += cycles;
    }

    pub fn a(&self) -> u8 {
        self.a
    }
    pub fn b(&self) -> u8 {
        self.b
    }
    pub fn c(&self) -> u8 {
        self.c
    }
    pub fn d(&self) -> u8 {
        self.d
    }
    pub fn e(&self) -> u8 {
        self.e
    }
    pub fn h(&self) -> u8 {
        self.h
    }

    pub fn af(&self) -> u16 {
        bytes_to_word(self.a, self.f)
    }
    pub fn bc(&self) -> u16 {
        bytes_to_word(self.b, self.c)
    }
    pub fn de(&self) -> u16 {
        bytes_to_word(self.d, self.e)
    }
    pub fn hl(&self) -> u16 {
        bytes_to_word(self.h, self.l)
    }

    pub fn stack_pointer(&self) -> u16 {
        self.sp
    }
    pub fn set_a(&mut self, val: u8) {
        self.a = val;
    }

    pub fn set_c(&mut self, val: u8) {
        self.c = val;
    }

    pub fn set_bc(&mut self, val: u16) {
        let (b, c) = word_to_bytes(val);
        self.b = b;
        self.c = c;
    }

    pub fn set_de(&mut self, val: u16) {
        let (d, e) = word_to_bytes(val);
        self.d = d;
        self.e = e;
    }

    pub fn set_sp(&mut self, val: u16) {
        self.sp = val;
    }

    pub fn set_hl(&mut self, val: u16) {
        let (h, l) = word_to_bytes(val);
        self.h = h;
        self.l = l;
    }

    pub fn set_h(&mut self, val: u8) {
        self.h = val;
    }

    pub fn frame_elapsed(&mut self, fps: u64) -> bool {
        // Gameboy runs at 4_200_000 hz
        let target_cycles = 4_200_000 / (fps as usize);

        if self.cycles > target_cycles {
            self.cycles -= target_cycles;
            true
        } else {
            false
        }
    }

    pub fn add_a(&mut self, val: u8, with_carry: bool) {
        let val = val + if with_carry && self.flags.c() { 1 } else { 0 };
        let half_carry = ((self.a & 0xf) + (val & 0xf)) & 0x10 > 0;
        let (new_a, carry) = self.a.overflowing_add(val);

        self.flags.update_half_carry(half_carry);
        self.flags.update_carry(carry);
        self.flags.update_zero(new_a == 0);
        self.flags.clear_subtract();

        self.a = new_a;
    }

    pub fn cmp_a_with(&mut self, val: u8) {
        let r = self.a.wrapping_sub(val);
        self.flags.update_zero(r == 0);
        self.flags.update_half_carry((self.a & 0x0F) < (val & 0x0F));
        self.flags.set_subtract();
        self.flags.update_carry(self.a < val);
    }

    pub fn dump(&self) {
        println!("af = [0x{a:02X}, 0x{f:02X}], bc = [0x{b:02X}, 0x{c:02X}], de = [0x{d:02X}, 0x{e:02X}], hl = [0x{h:02x}, 0x{l:02X}], sp = 0x{sp:04X}, pc = 0x{pc:04X}",
                 a = self.a,
                 b = self.b,
                 c = self.c,
                 d = self.d,
                 e = self.e,
                 f = self.f,
                 h = self.h,
                 l = self.l,
                 sp = self.sp,
                 pc = self.pc);
    }

    pub fn pop_stack(&mut self, memory: &mut Memory) -> u16 {
        self.sp += 2;
        memory.read_word(self.sp)
    }

    pub fn push_stack(&mut self, memory: &mut Memory, value: u16) {
        memory.write_word(self.sp, value);
        self.sp -= 2;
    }
}

pub struct Flags(u8);

const BITMASK_ZERO: u8 = 0b1000_0000;
const BITMASK_SUBTRACT: u8 = 0b0100_0000;
const BITMASK_HALF_CARRY: u8 = 0b0010_0000;
const BITMASK_CARRY: u8 = 0b0001_0000;

impl Flags {
    /// Zero Flag. This bit is set when the result of a math operation is zero or two values match
    /// when using the CP instruction.
    pub fn z(&self) -> bool {
        (self.0 & BITMASK_ZERO) > 0
    }

    /// Subtract flag. This bit is set if a subtraction was performed in the last math instruction.
    pub fn n(&self) -> bool {
        (self.0 & BITMASK_SUBTRACT) > 0
    }

    /// Half carry flag. This bit is set if a carry occurred from the lower nibble in the last math
    /// operation.
    pub fn h(&self) -> bool {
        (self.0 & BITMASK_HALF_CARRY) > 0
    }

    /// Carry flag. This bit is set if a carry occurred from the last math operation or if register
    /// A is the smaller value when executing the CP instruction.
    pub fn c(&self) -> bool {
        (self.0 & BITMASK_CARRY) > 0
    }

    fn update(&mut self, mask: u8, set: bool) {
        if set {
            self.0 = self.0 | mask;
        } else {
            self.0 = self.0 & !mask;
        }
    }

    pub fn set_zero(&mut self) {
        self.update_zero(true);
    }

    pub fn set_half_carry(&mut self) {
        self.update_half_carry(true);
    }

    pub fn set_subtract(&mut self) {
        self.update_subtract(true);
    }

    pub fn clear_subtract(&mut self) {
        self.update_subtract(false);
    }

    pub fn clear_half_carry(&mut self) {
        self.update_half_carry(false);
    }

    pub fn clear_carry(&mut self) {
        self.update_carry(false);
    }

    pub fn update_zero(&mut self, zero: bool) {
        self.update(BITMASK_ZERO, zero);
    }

    pub fn update_subtract(&mut self, subtract: bool) {
        self.update(BITMASK_SUBTRACT, subtract);
    }

    pub fn update_half_carry(&mut self, half_carry: bool) {
        self.update(BITMASK_HALF_CARRY, half_carry);
    }

    pub fn update_carry(&mut self, carry: bool) {
        self.update(BITMASK_CARRY, carry);
    }
}
