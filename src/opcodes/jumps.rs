use crate::Cpu;
use crate::Memory;

pub fn rst_38(memory: &mut Memory, cpu: &mut Cpu) {
    // RST 38H 1 16 - - - -
    cpu.clock_cycles(16);
    cpu.increment_program_counter();
    cpu.push_stack(memory, cpu.program_counter());
    cpu.set_program_counter(38);
}

pub fn call_z_a16(memory: &mut Memory, cpu: &mut Cpu) {
    // 0xCC CALL Z, a16 3 24/12- - - -
    cpu.increment_program_counter();

    if cpu.flags.z() {
        cpu.push_stack(memory, cpu.program_counter());

        cpu.clock_cycles(24);
        let address = memory.read_word(cpu.program_counter());
        cpu.set_program_counter(address);
    } else {
        cpu.clock_cycles(16);
        cpu.increment_program_counter();
        cpu.increment_program_counter();
    }
}

pub fn jr_nz_r8(memory: &mut Memory, cpu: &mut Cpu) {
    // 0x20 JR NZ, r8 2 12/8 - - - -
    cpu.increment_program_counter();
    let val = memory.read_byte(cpu.program_counter()) as i8;

    cpu.increment_program_counter();
    if !cpu.flags.z() {
        let program_counter = if val < 0 {
            cpu.program_counter() - (val.abs() as u16)
        } else {
            cpu.program_counter() + (val as u16)
        };
        cpu.set_program_counter(program_counter);
        cpu.clock_cycles(12);
    } else {
        cpu.clock_cycles(8);
    }
}

pub fn jr_z_r8(memory: &mut Memory, cpu: &mut Cpu) {
    // 0x28 JR Z, r8 2 12/8 - - - -
    cpu.increment_program_counter();
    let val = memory.read_byte(cpu.program_counter()) as i8;

    cpu.increment_program_counter();
    if cpu.flags.z() {
        let program_counter = if val < 0 {
            cpu.program_counter() - (val.abs() as u16)
        } else {
            cpu.program_counter() + (val as u16)
        };
        cpu.set_program_counter(program_counter);
        cpu.clock_cycles(12);
    } else {
        cpu.clock_cycles(8);
    }
}

pub fn jr_r8(memory: &mut Memory, cpu: &mut Cpu) {
    // 0x18 JR r8 2 12 - - - -
    cpu.increment_program_counter();
    let val = memory.read_byte(cpu.program_counter()) as i8;

    cpu.increment_program_counter();
    let program_counter = if val < 0 {
        cpu.program_counter() - (val.abs() as u16)
    } else {
        cpu.program_counter() + (val as u16)
    };
    cpu.set_program_counter(program_counter);
    cpu.clock_cycles(12);
}

pub fn call_a16(memory: &mut Memory, cpu: &mut Cpu) {
    // 0xCD CALL a16 3 24 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(24);

    let address = memory.read_word(cpu.program_counter());
    cpu.increment_program_counter();

    cpu.push_stack(memory, cpu.program_counter());
    cpu.set_program_counter(address);
}

pub fn jp_a16(memory: &mut Memory, cpu: &mut Cpu) {
    // 0xC3 JP a16 3 16 - - - -
    cpu.clock_cycles(16);
    cpu.increment_program_counter();

    let address = memory.read_word(cpu.program_counter());

    cpu.set_program_counter(address);
}
