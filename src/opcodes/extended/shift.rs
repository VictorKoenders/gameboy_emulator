use crate::Cpu;
use crate::Memory;

pub fn rl_c(_: &mut Memory, cpu: &mut Cpu) {
    // 0x11 RL C 1 4 Z 0 0 C
    cpu.increment_program_counter();
    cpu.clock_cycles(4);

    let c = rl(cpu, cpu.c());
    cpu.flags.update_zero(c == 0);
    cpu.set_c(c);
}

fn rl(cpu: &mut Cpu, val: u8) -> u8 {
    let new_carry = (val & 0b1000_0000) > 0;
    let mut new_val = val.wrapping_shl(1);
    if cpu.flags.c() {
        new_val |= 0b0000_0001;
    }
    cpu.flags.clear_half_carry();
    cpu.flags.clear_subtract();
    cpu.flags.update_carry(new_carry);

    new_val
}
