use crate::{Cpu, Memory};

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
    cpu.flags.set_subtract();
    cpu.flags.update_zero(c == 1);
    cpu.flags.update_half_carry(c == 0x10);
    cpu.set_c(c.wrapping_sub(1));
}

pub fn dec_b(_: &mut Memory, cpu: &mut Cpu) {
    // 0x05 DEC B 1 4 Z 1 H -
    cpu.clock_cycles(4);
    cpu.increment_program_counter();

    let b = cpu.b();
    cpu.flags.update_zero(b == 1);
    cpu.flags.update_half_carry(b == 0x10);
    cpu.flags.set_subtract();

    cpu.set_b(b.wrapping_sub(1));
}

pub fn dec_a(_: &mut Memory, cpu: &mut Cpu) {
    // 0x3D DEC A 1 4 Z 1 H -
    cpu.clock_cycles(4);
    cpu.increment_program_counter();

    let a = cpu.a();
    cpu.flags.update_zero(a == 1);
    cpu.flags.update_half_carry(a == 0x10);
    cpu.flags.set_subtract();

    cpu.set_a(a.wrapping_sub(1));
}

pub fn dec_e(_: &mut Memory, cpu: &mut Cpu) {
    // 0x1D DEC E 1 4 Z 1 H -
    cpu.clock_cycles(4);
    cpu.increment_program_counter();

    let e = cpu.e();
    cpu.flags.update_zero(e == 1);
    cpu.flags.update_half_carry(e == 0x10);
    cpu.flags.set_subtract();

    cpu.set_e(e.wrapping_sub(1));
}

pub fn dec_d(_: &mut Memory, cpu: &mut Cpu) {
    // 0x16 DEC D 1 4 Z 1 H -
    cpu.clock_cycles(4);
    cpu.increment_program_counter();

    let d = cpu.d();
    cpu.flags.update_zero(d == 1);
    cpu.flags.update_half_carry(d == 0x10);
    cpu.flags.set_subtract();

    cpu.set_d(d.wrapping_sub(1));
}
pub fn sub_b(_: &mut Memory, cpu: &mut Cpu) {
    // 0x90 SUB B 1 4 Z 1 H C
    cpu.increment_program_counter();
    cpu.clock_cycles(4);

    let b = cpu.b();
    do_sub(cpu, b);
}

fn do_sub(cpu: &mut Cpu, val: u8) {
    let a = cpu.a();
    let new_a = a.wrapping_sub(val);

    let carry_bit = (a as u16) ^ (val as u16) ^ (new_a as u16);

    cpu.flags.update_zero(new_a == 0);
    cpu.flags.update_carry(carry_bit & 0x100 > 0);
    cpu.flags.set_subtract();
    cpu.flags.update_half_carry(carry_bit & 0x10 > 0);

    cpu.set_a(new_a);
}
