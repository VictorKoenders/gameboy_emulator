use crate::cpu::Cpu;
use crate::memory::Memory;

pub fn add_a_e(_: &mut Memory, cpu: &mut Cpu) {
    // ADD A,E 1 4 Z 0 H C
    cpu.clock_cycles(4);
    cpu.increment_program_counter();

    let val = cpu.e();

    cpu.add_a(val, false);
}

pub fn adc_a_d8(memory: &mut Memory, cpu: &mut Cpu) {
    // ADC A,d8 2 8 Z 0 H C
    cpu.clock_cycles(8);
    cpu.increment_program_counter();

    let val = memory.read_byte(cpu.program_counter());
    cpu.increment_program_counter();

    cpu.add_a(val, true);
}

pub fn inc_bc(_: &mut Memory, cpu: &mut Cpu) {
    // INC BC 1 8 - - - -
    cpu.clock_cycles(8);
    cpu.increment_program_counter();

    cpu.set_bc(cpu.bc().wrapping_add(1));
}

pub fn inc_c(_: &mut Memory, cpu: &mut Cpu) {
    // INC C 1 4 Z 0 H -
    cpu.clock_cycles(4);
    cpu.increment_program_counter();

    let val = cpu.c();
    cpu.flags.clear_subtract();
    if val == std::u8::MAX {
        cpu.flags.set_zero();
        cpu.flags.clear_half_carry();
        cpu.set_c(0);
    } else {
        cpu.flags.update_half_carry(val == 0b1111);
        cpu.set_c(val + 1);
    }
}

pub fn adc_a_b(_: &mut Memory, cpu: &mut Cpu) {
    // ADC A, B 1 4 Z 0 H C
    cpu.clock_cycles(4);
    cpu.increment_program_counter();

    let val = cpu.b();
    cpu.add_a(val, true);
}

pub fn adc_a_c(_: &mut Memory, cpu: &mut Cpu) {
    // ADC A, C 1 4 Z 0 H C
    cpu.clock_cycles(4);
    cpu.increment_program_counter();

    let val = cpu.c();
    cpu.add_a(val, true);
}
