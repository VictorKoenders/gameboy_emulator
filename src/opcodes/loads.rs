use crate::{Cpu, Memory};

pub fn ld_h_ptr_hl(memory: &mut Memory, cpu: &mut Cpu) {
    // 0x66 LD H,(HL) 1 8 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(18);

    let hl = cpu.hl();
    let val = memory.read_byte(hl);
    cpu.set_h(val);
}

pub fn ld_ptr_hl_e(memory: &mut Memory, cpu: &mut Cpu) {
    // LD (HL),E 1 8 - - - -
    cpu.clock_cycles(8);
    cpu.increment_program_counter();

    let val = cpu.e();
    let address = cpu.hl();

    memory.write_byte(address, val);
}

pub fn ld_ptr_a16_sp(memory: &mut Memory, cpu: &mut Cpu) {
    // LD (a16), SP 3 20 - - - -
    cpu.clock_cycles(20);
    cpu.increment_program_counter();
    let address = memory.read_word(cpu.program_counter());
    cpu.increment_program_counter();
    cpu.increment_program_counter();

    memory.write_word(address, cpu.stack_pointer());
}

pub fn ld_c_d8(memory: &mut Memory, cpu: &mut Cpu) {
    // LD C, d8 2 8 - - - -
    cpu.clock_cycles(8);
    cpu.increment_program_counter();

    let val = memory.read_byte(cpu.program_counter());
    cpu.increment_program_counter();

    cpu.set_c(val);
}

pub fn ld_ptr_a16_a(memory: &mut Memory, cpu: &mut Cpu) {
    // 0xEA LD (a16), A 3 16 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(16);
    let address = memory.read_word(cpu.program_counter());
    cpu.increment_program_counter();
    cpu.increment_program_counter();

    memory.write_byte(address, cpu.a());
}

pub fn ldh_ptr_a8_a(memory: &mut Memory, cpu: &mut Cpu) {
    // 0xE0 LDH (a8), A 2 12 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(12);
    let val = memory.read_byte(cpu.program_counter());
    cpu.increment_program_counter();

    memory.write_byte(0xFF00 + (val as u16), cpu.a());
}

pub fn ld_a_d8(memory: &mut Memory, cpu: &mut Cpu) {
    // 0x3E LD A, d8 2 8 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(8);

    let value = memory.read_byte(cpu.program_counter());
    cpu.increment_program_counter();

    cpu.set_a(value);
}

pub fn ldh_a_ptr_a8(memory: &mut Memory, cpu: &mut Cpu) {
    // 0xF0 LDH A, (a8) 2 12 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(12);
    let address = memory.read_byte(cpu.program_counter());
    cpu.increment_program_counter();

    let address = 0xFF00 + (address as u16);
    let value = memory.read_byte(address);
    cpu.set_a(value);
}

pub fn ld_sp_d16(memory: &mut Memory, cpu: &mut Cpu) {
    // 0x31 LD SP, d16 3 12 - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(12);

    let val = memory.read_word(cpu.program_counter());
    cpu.increment_program_counter();
    cpu.increment_program_counter();

    cpu.set_sp(val);
}

pub fn ld_hl_d16(memory: &mut Memory, cpu: &mut Cpu) {
    // 0x21 LD HL, d16 3 12 - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(12);

    let val = memory.read_word(cpu.program_counter());
    cpu.increment_program_counter();
    cpu.increment_program_counter();

    cpu.set_hl(val);
}

pub fn ld_ptr_hl_minus_a(memory: &mut Memory, cpu: &mut Cpu) {
    // 0x32 LD (HL -), A 1 8 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(8);
    let address = cpu.hl();
    let val = cpu.a();

    cpu.set_hl(address.wrapping_sub(1));
    memory.write_byte(address, val);
}

pub fn ld_ptr_c_a(memory: &mut Memory, cpu: &mut Cpu) {
    // 0xE2 LD (C), A 1 8 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(8);

    let c = cpu.c();
    let val = cpu.a();
    let addr = (0xff00) | (c as u16);

    memory.write_byte(addr, val);
}

pub fn ld_ptr_hl_a(memory: &mut Memory, cpu: &mut Cpu) {
    // 0x77 LD (HL), A 1 8 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(8);

    let address = cpu.hl();
    let val = cpu.a();

    memory.write_byte(address, val);
}

pub fn ld_de_d16(memory: &mut Memory, cpu: &mut Cpu) {
    // 0x11 LD DE, d16 3 12 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(12);

    let word = memory.read_word(cpu.program_counter());
    cpu.increment_program_counter();
    cpu.increment_program_counter();

    cpu.set_de(word);
}

pub fn ld_a_ptr_de(memory: &mut Memory, cpu: &mut Cpu) {
    // 0x1A LD A, (DE) 1 8 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(8);

    let val = memory.read_byte(cpu.de());
    cpu.set_a(val);
}

pub fn ld_c_a(_: &mut Memory, cpu: &mut Cpu) {
    // 0x4F LD C, A 1 4 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(4);

    cpu.set_a(cpu.c());
}

pub fn ld_b_d8(memory: &mut Memory, cpu: &mut Cpu) {
    // 0x06 LD B, d8 2 8 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(8);

    let val = memory.read_byte(cpu.program_counter());
    cpu.increment_program_counter();

    cpu.set_b(val);
}

pub fn ld_ptr_hl_plus_a(memory: &mut Memory, cpu: &mut Cpu) {
    // 0x22 LD (HL+), A 1 8 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(8);

    let hl_address = cpu.hl();
    let a = cpu.a();

    memory.write_byte(hl_address, a);
    cpu.set_hl(hl_address.wrapping_add(1));
}

pub fn ld_a_e(_: &mut Memory, cpu: &mut Cpu) {
    // 0x7B LD A, E 1 4 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(4);

    let e = cpu.e();
    cpu.set_a(e);
}

pub fn ld_l_d8(memory: &mut Memory, cpu: &mut Cpu) {
    // 0x2E LD L, d8 2 8 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(8);

    let val = memory.read_byte(cpu.program_counter());
    cpu.increment_program_counter();

    cpu.set_l(val);
}

pub fn ld_h_a(_: &mut Memory, cpu: &mut Cpu) {
    // 0x67 LD H, A 1 8 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(8);

    let val = cpu.a();
    cpu.set_h(val);
}

pub fn ld_d_a(_: &mut Memory, cpu: &mut Cpu) {
    // 0x57 LD D, A 1 4 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(8);

    let val = cpu.a();
    cpu.set_d(val);
}

pub fn ld_e_d8(memory: &mut Memory, cpu: &mut Cpu) {
    // 0x1E LD E, d8 2 8 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(8);

    let val = memory.read_byte(cpu.program_counter());
    cpu.increment_program_counter();

    cpu.set_e(val);
}

pub fn ld_a_h(_: &mut Memory, cpu: &mut Cpu) {
    // 0x7C LD A, H 1 4 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(4);

    let val = cpu.h();
    cpu.set_h(val);
}

pub fn ld_d_d8(memory: &mut Memory, cpu: &mut Cpu) {
    // 0x16 LD D, d8 2 8 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(8);

    let val = memory.read_byte(cpu.program_counter());
    cpu.increment_program_counter();

    cpu.set_d(val);
}

pub fn ld_a_l(_: &mut Memory, cpu: &mut Cpu) {
    // 0x7D LD A, L 1 4 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(4);

    let val = cpu.l();
    cpu.set_a(val);
}

pub fn ld_a_b(_: &mut Memory, cpu: &mut Cpu) {
    // 0x78 LD A, B 1 4 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(4);

    let val = cpu.b();
    cpu.set_a(val);
}

pub fn ld_b_a(_: &mut Memory, cpu: &mut Cpu) {
    // 0x47 LD B, A 1 4 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(4);

    let val = cpu.a();
    cpu.set_b(val);
}

