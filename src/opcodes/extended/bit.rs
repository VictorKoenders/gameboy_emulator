use crate::memory::Memory;
use crate::cpu::Cpu;

pub fn bit_7_h(_memory: &mut Memory, cpu: &mut Cpu) {
    // 0x7C BIT 7, H 1 4 z 0 1 -
    cpu.increment_program_counter();
    cpu.clock_cycles(4);

    println!("{:b}", cpu.h());
    cpu.flags.update_zero(cpu.h() & 0b1000_0000 == 0);
    cpu.flags.clear_subtract();
    cpu.flags.set_half_carry();
}


