use crate::cpu::Cpu;
use crate::memory::Memory;

pub fn xor_a(_: &mut Memory, cpu: &mut Cpu) {
    // 0xAF XOR A 1 4 Z 0 0 0
    cpu.increment_program_counter();
    cpu.clock_cycles(4);

    cpu.set_a(0);
    cpu.flags.set_zero();
    cpu.flags.clear_half_carry();
    cpu.flags.clear_subtract();
    cpu.flags.clear_carry();
}
