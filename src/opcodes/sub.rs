use crate::cpu::Cpu;
use crate::memory::Memory;

pub fn dec_bc(_: &mut Memory, cpu: &mut Cpu) {
    // 0x0B DEC BC 1 8 - - - -
    cpu.clock_cycles(8);
    cpu.increment_program_counter();
    cpu.set_bc(cpu.bc().wrapping_sub(1));
}

pub fn dec_c(_: &mut Memory, cpu: &mut Cpu) {
    // 0x0D DEC C 1 4 Z 1 H -
    cpu.clock_cycles(4);
    cpu.increment_program_counter();
    let c = cpu.c();
    cpu.flags.update_zero(c == 1);
    cpu.flags.update_half_carry(c == 0x10);
    cpu.set_c(c.wrapping_sub(1));
}
