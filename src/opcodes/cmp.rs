use crate::{Cpu, Memory};

pub fn cp_d8(memory: &mut Memory, cpu: &mut Cpu) {
    // CP d8 2 8 Z 1 H C
    cpu.clock_cycles(8);
    cpu.increment_program_counter();
    let cmp_val = memory.read_byte(cpu.program_counter());
    cpu.increment_program_counter();

    cpu.cmp_a_with(cmp_val);
}

pub fn cp_ptr_hl(memory: &mut Memory, cpu: &mut Cpu) {
    // 0xBE CP (HL) 1 8 Z 1 H C
    cpu.clock_cycles(8);
    cpu.increment_program_counter();

    let val = memory.read_byte(cpu.hl());

    cpu.cmp_a_with(val);
}

