use crate::Cpu;
use crate::Memory;

pub fn bit_7_h(_memory: &mut Memory, cpu: &mut Cpu) {
    // 0x7C BIT 7, H 1 4 z 0 1 -
    cpu.increment_program_counter();
    cpu.clock_cycles(4);

    cpu.flags.update_zero(bit_cleared(cpu.h(), 7));
    cpu.flags.clear_subtract();
    cpu.flags.set_half_carry();
}

fn bit_cleared(value: u8, offset: i8) -> bool {
    !bit_set(value, offset)
}

fn bit_set(value: u8, offset: i8) -> bool {
    (value >> offset) & 1 == 1
}
