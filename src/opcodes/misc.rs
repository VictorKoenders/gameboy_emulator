use crate::Cpu;
use crate::Memory;

pub fn push_bc(memory: &mut Memory, cpu: &mut Cpu) {
    // 0xC5 PUSH BC 1 16 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(16);

    cpu.push_stack(memory, cpu.bc());
}

pub fn pop_bc(memory: &mut Memory, cpu: &mut Cpu) {
    // 0xC1 POP BC 1 12 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(12);

    let bc = cpu.pop_stack(memory);
    cpu.set_bc(bc);
}

pub fn rla(_: &mut Memory, cpu: &mut Cpu) {
    // 0x17 RLA 1 4 0 0 0 C
    cpu.increment_program_counter();
    cpu.clock_cycles(4);

    let val = cpu.a();
    let new_carry = (val & 0b1000_0000) > 0;
    let mut new_val = val.wrapping_shl(1);
    if cpu.flags.c() {
        new_val |= 0b0000_0001;
    }
    cpu.flags.clear_zero();
    cpu.flags.clear_half_carry();
    cpu.flags.clear_subtract();
    cpu.flags.update_carry(new_carry);

    cpu.set_a(new_val);
}
