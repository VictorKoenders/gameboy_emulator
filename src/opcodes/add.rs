use crate::{Cpu, Memory};

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
    if val == core::u8::MAX {
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

pub fn add_a_ptr_hl(memory: &mut Memory, cpu: &mut Cpu) {
    // 0x86 ADD A, (HL) 1 8 Z 0 H C
    cpu.clock_cycles(4);
    cpu.increment_program_counter();

    let addr = cpu.hl();
    let val = memory.read_byte(addr);
    cpu.add_a(val, false);
}

pub fn inc_hl(_: &mut Memory, cpu: &mut Cpu) {
    // 0x23 INC HL 1 8 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(8);
    cpu.set_hl(cpu.hl().wrapping_add(1));
}

pub fn inc_de(_: &mut Memory, cpu: &mut Cpu) {
    // 0x13 DE HL 1 8 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(8);
    cpu.set_de(cpu.de().wrapping_add(1));
}

pub fn inc_b(_: &mut Memory, cpu: &mut Cpu) {
    // 0x04 INC B 1 4 Z 0 H -
    cpu.increment_program_counter();
    cpu.clock_cycles(4);

    let b = cpu.b();
    cpu.flags.update_zero(b == 0xff);
    cpu.flags.clear_subtract();
    cpu.flags.update_half_carry(b == 0x80);

    cpu.set_b(b.wrapping_add(1));
}

pub fn inc_h(_: &mut Memory, cpu: &mut Cpu) {
    // 0x24 INC H 1 4 Z 0 H -
    cpu.increment_program_counter();
    cpu.clock_cycles(4);

    let h = cpu.h();
    cpu.flags.update_zero(h == 0xff);
    cpu.flags.clear_subtract();
    cpu.flags.update_half_carry(h == 0x80);

    cpu.set_h(h.wrapping_add(1));
}
