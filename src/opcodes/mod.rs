use crate::Cpu;
use crate::Memory;

#[macro_export]
macro_rules! unimpl_opcode {
    ($opcode:tt $($descr:tt)*) => {{
        fn unimplemented_opcode(_memory: &mut Memory, cpu: &mut Cpu) {
            panic!("Not implemented:\n${:04X}: 0x{:02X} {}", cpu.program_counter(), $opcode, stringify!($($descr)*));
        }
        (stringify!($opcode, $($descr)*), unimplemented_opcode)
    }}
}

mod extended;

mod add;
mod cmp;
mod jumps;
mod loads;
mod misc;
mod sub;
mod xor;

pub fn execute(memory: &mut Memory, cpu: &mut Cpu) {
    let instruction = memory.read_byte(cpu.program_counter());
    let (_name, function) = INSTRUCTIONS[instruction as usize];

    (function)(memory, cpu);

    if cpu.program_counter() == 0x0100 {
        memory.disable_bios();
    }
}

pub type Op = fn(memory: &mut Memory, cpu: &mut Cpu);

fn nop(_: &mut Memory, cpu: &mut Cpu) {
    cpu.clock_cycles(4);
    cpu.increment_program_counter();
}

const INSTRUCTIONS: [(&str, Op); 256] = [
    // 0x0x
    ("nop", nop),
    unimpl_opcode!(0x01 LD BC,d16 3 12 - - - -),
    unimpl_opcode!(0x02 LD (BC),A 1 8 - - - -),
    ("INC bc", add::inc_bc),
    unimpl_opcode!(0x04 INC B 1 4 Z 0 H -),
    unimpl_opcode!(0x05 DEC B 1 4 Z 1 H -),
    ("0x06 LD B, d8", loads::ld_b_d8),
    unimpl_opcode!(0x07 RLCA 1 4 0 0 0 C),
    ("LD (a16), sp", loads::ld_ptr_a16_sp),
    unimpl_opcode!(0x09 ADD HL,BC 1 8 - 0 H C),
    unimpl_opcode!(0x0A LD A,(BC) 1 8 - - - -),
    ("DEC bc", sub::dec_bc),
    ("INC c", add::inc_c),
    ("DEC c", sub::dec_c),
    ("LD c, d8", loads::ld_c_d8),
    unimpl_opcode!(0x0F RRCA 1 4 0 0 0 C),
    // 0x1x
    unimpl_opcode!(0x10 STOP 0 1 4 - - - -), // actually 1 byte size https://stackoverflow.com/a/41422692
    ("LD DE, d16", loads::ld_de_d16),
    unimpl_opcode!(0x12 LD (DE),A 1 8 - - - -),
    unimpl_opcode!(0x13 INC DE 1 8 - - - -),
    unimpl_opcode!(0x14 INC D 1 4 Z 0 H -),
    unimpl_opcode!(0x15 DEC D 1 4 Z 1 H -),
    unimpl_opcode!(0x16 LD D,d8 2 8 - - - -),
    ("0x17 RLA", misc::rla),
    ("JR r8", jumps::jr_r8),
    unimpl_opcode!(0x19 ADD HL,DE 1 8 - 0 H C),
    ("LD A, (DE)", loads::ld_a_ptr_de),
    unimpl_opcode!(0x1B DEC DE 1 8 - - - -),
    unimpl_opcode!(0x1C INC E 1 4 Z 0 H -),
    unimpl_opcode!(0x1D DEC E 1 4 Z 1 H -),
    unimpl_opcode!(0x1E LD E,d8 2 8 - - - -),
    unimpl_opcode!(0x1F RRA 1 4 0 0 0 C),
    // 0x2x
    ("0x20 JR NZ,r8", jumps::jr_nz_r8),
    ("LD HL, d16", loads::ld_hl_d16),
    unimpl_opcode!(0x22 LD (HL+),A 1 8 - - - -),
    unimpl_opcode!(0x23 INC HL 1 8 - - - -),
    unimpl_opcode!(0x24 INC H 1 4 Z 0 H -),
    unimpl_opcode!(0x25 DEC H 1 4 Z 1 H -),
    unimpl_opcode!(0x26 LD H,d8 2 8 - - - -),
    unimpl_opcode!(0x27 DAA 1 4 Z - 0 C),
    ("JR z, r8", jumps::jr_z_r8),
    unimpl_opcode!(0x29 ADD HL,HL 1 8 - 0 H C),
    unimpl_opcode!(0x2A LD A,(HL+) 1 8 - - - -),
    unimpl_opcode!(0x2B DEC HL 1 8 - - - -),
    unimpl_opcode!(0x2C INC L 1 4 Z 0 H -),
    unimpl_opcode!(0x2D DEC L 1 4 Z 1 H -),
    unimpl_opcode!(0x2E LD L,d8 2 8 - - - -),
    unimpl_opcode!(0x2F CPL 1 4 - 1 1 -),
    // 0x3x
    unimpl_opcode!(0x30 JR NC,r8 2 12/8 - - - -),
    ("LD sp, d16", loads::ld_sp_d16),
    ("LD (HL-), A", loads::ld_ptr_hl_minus_a),
    unimpl_opcode!(0x33 INC SP 1 8 - - - -),
    unimpl_opcode!(0x34 INC (HL) 1 12 Z 0 H -),
    unimpl_opcode!(0x35 DEC (HL) 1 12 Z 1 H -),
    unimpl_opcode!(0x36 LD (HL),d8 2 12 - - - -),
    unimpl_opcode!(0x37 SCF 1 4 - 0 0 1),
    unimpl_opcode!(0x38 JR C,r8 2 12/8 - - - -),
    unimpl_opcode!(0x39 ADD HL,SP 1 8 - 0 H C),
    unimpl_opcode!(0x3A LD A,(HL-) 1 8 - - - -),
    unimpl_opcode!(0x3B DEC SP 1 8 - - - -),
    unimpl_opcode!(0x3C INC A 1 4 Z 0 H -),
    unimpl_opcode!(0x3D DEC A 1 4 Z 1 H -),
    ("LD a, d8", loads::ld_a_d8),
    unimpl_opcode!(0x3F CCF14- 0 0 C),
    // 0x4x
    unimpl_opcode!(0x40 LD B,B 1 4 - - - -),
    unimpl_opcode!(0x41 LD B,C 1 4 - - - -),
    unimpl_opcode!(0x42 LD B,D 1 4 - - - -),
    unimpl_opcode!(0x43 LD B,E 1 4 - - - -),
    unimpl_opcode!(0x44 LD B,H 1 4 - - - -),
    unimpl_opcode!(0x45 LD B,L 1 4 - - - -),
    unimpl_opcode!(0x46 LD B,(HL)1 8 - - - -),
    unimpl_opcode!(0x47 LD B,A 1 4 - - - -),
    unimpl_opcode!(0x48 LD C,B 1 4 - - - -),
    unimpl_opcode!(0x49 LD C,C 1 4 - - - -),
    unimpl_opcode!(0x4A LD C,D 1 4 - - - -),
    unimpl_opcode!(0x4B LD C,E 1 4 - - - -),
    unimpl_opcode!(0x4C LD C,H 1 4 - - - -),
    unimpl_opcode!(0x4D LD C,L 1 4 - - - -),
    unimpl_opcode!(0x4E LD C,(HL) 1 8 - - - -),
    ("0x4F LD C,A", loads::ld_c_a),
    // 0x5x
    unimpl_opcode!(0x50 LD D,B 1 4 - - - -),
    unimpl_opcode!(0x51 LD D,C 1 4 - - - -),
    unimpl_opcode!(0x52 LD D,D 1 4 - - - -),
    unimpl_opcode!(0x53 LD D,E 1 4 - - - -),
    unimpl_opcode!(0x54 LD D,H 1 4 - - - -),
    unimpl_opcode!(0x55 LD D,L 1 4 - - - -),
    unimpl_opcode!(0x56 LD D,(HL) 1 8 - - - -),
    unimpl_opcode!(0x57 LD D,A 1 4 - - - -),
    unimpl_opcode!(0x58 LD E,B 1 4 - - - -),
    unimpl_opcode!(0x59 LD E,C 1 4 - - - -),
    unimpl_opcode!(0x5A LD E,D 1 4 - - - -),
    unimpl_opcode!(0x5B LD E,E 1 4 - - - -),
    unimpl_opcode!(0x5C LD E,H 1 4 - - - -),
    unimpl_opcode!(0x5D LD E,L 1 4 - - - -),
    unimpl_opcode!(0x5E LD E,(HL) 1 8 - - - -),
    unimpl_opcode!(0x5F LD E,A 1 4 - - - -),
    // 0x6x
    unimpl_opcode!(0x60 LD H,B 1 4 - - - -),
    unimpl_opcode!(0x61 LD H,C 1 4 - - - -),
    unimpl_opcode!(0x62 LD H,D 1 4 - - - -),
    unimpl_opcode!(0x63 LD H,E 1 4 - - - -),
    unimpl_opcode!(0x64 LD H,H 1 4 - - - -),
    unimpl_opcode!(0x65 LD H,L 1 4 - - - -),
    ("LD h, (hl)", loads::ld_h_ptr_hl),
    unimpl_opcode!(0x67 LD H,A 1 4 - - - -),
    unimpl_opcode!(0x68 LD L,B 1 4 - - - -),
    unimpl_opcode!(0x69 LD L,C 1 4 - - - -),
    unimpl_opcode!(0x6A LD L,D 1 4 - - - -),
    unimpl_opcode!(0x6B LD L,E 1 4 - - - -),
    unimpl_opcode!(0x6C LD L,H 1 4 - - - -),
    unimpl_opcode!(0x6D LD L,L 1 4 - - - -),
    unimpl_opcode!(0x6E LD L,(HL) 1 8 - - - -),
    unimpl_opcode!(0x6F LD L,A 1 4 - - - -),
    // 0x7x
    unimpl_opcode!(0x70 LD (HL),B18- - - -),
    unimpl_opcode!(0x71 LD (HL),C18- - - -),
    unimpl_opcode!(0x72 LD (HL),D18- - - -),
    ("LD (hl), e", loads::ld_ptr_hl_e),
    unimpl_opcode!(0x74 LD (HL),H18- - - -),
    unimpl_opcode!(0x75 LD (HL),L18- - - -),
    unimpl_opcode!(0x76 HALT14- - - -),
    ("LD (HL),A", loads::ld_ptr_hl_a),
    unimpl_opcode!(0x78 LD A,B14- - - -),
    unimpl_opcode!(0x79 LD A,C14- - - -),
    unimpl_opcode!(0x7A LD A,D14- - - -),
    unimpl_opcode!(0x7B LD A,E14- - - -),
    unimpl_opcode!(0x7C LD A,H14- - - -),
    unimpl_opcode!(0x7D LD A,L14- - - -),
    unimpl_opcode!(0x7E LD A,(HL)18- - - -),
    unimpl_opcode!(0x7F LD A,A14- - - -),
    // 0x8x
    unimpl_opcode!(0x80 ADD A,B14Z 0 H C),
    unimpl_opcode!(0x81 ADD A,C14Z 0 H C),
    unimpl_opcode!(0x82 ADD A,D14Z 0 H C),
    ("ADD a, e", add::add_a_e),
    unimpl_opcode!(0x84 ADD A,H 1 4 Z 0 H C),
    unimpl_opcode!(0x85 ADD A,L 1 4 Z 0 H C),
    unimpl_opcode!(0x86 ADD A,(HL) 1 8 Z 0 H C),
    unimpl_opcode!(0x87 ADD A,A 1 4 Z 0 H C),
    ("ADC a, b", add::adc_a_b),
    ("ADC a, c", add::adc_a_c),
    unimpl_opcode!(0x8A ADC A,D 1 4 Z 0 H C),
    unimpl_opcode!(0x8B ADC A,E 1 4 Z 0 H C),
    unimpl_opcode!(0x8C ADC A,H 1 4 Z 0 H C),
    unimpl_opcode!(0x8D ADC A,L 1 4 Z 0 H C),
    unimpl_opcode!(0x8E ADC A,(HL) 1 8 Z 0 H C),
    unimpl_opcode!(0x8F ADC A,A 1 4 Z 0 H C),
    // 0x9x
    unimpl_opcode!(0x90 SUB B14Z 1 H C),
    unimpl_opcode!(0x91 SUB C14Z 1 H C),
    unimpl_opcode!(0x92 SUB D14Z 1 H C),
    unimpl_opcode!(0x93 SUB E14Z 1 H C),
    unimpl_opcode!(0x94 SUB H14Z 1 H C),
    unimpl_opcode!(0x95 SUB L14Z 1 H C),
    unimpl_opcode!(0x96 SUB (HL)18Z 1 H C),
    unimpl_opcode!(0x97 SUB A14Z 1 H C),
    unimpl_opcode!(0x98 SBC A,B14Z 1 H C),
    unimpl_opcode!(0x99 SBC A,C14Z 1 H C),
    unimpl_opcode!(0x9A SBC A,D14Z 1 H C),
    unimpl_opcode!(0x9B SBC A,E14Z 1 H C),
    unimpl_opcode!(0x9C SBC A,H14Z 1 H C),
    unimpl_opcode!(0x9D SBC A,L14Z 1 H C),
    unimpl_opcode!(0x9E SBC A,(HL)18Z 1 H C),
    unimpl_opcode!(0x9F SBC A,A14Z 1 H C),
    // 0xAx
    unimpl_opcode!(0xA0 AND B 1 4 Z 0 1 0),
    unimpl_opcode!(0xA1 AND C 1 4 Z 0 1 0),
    unimpl_opcode!(0xA2 AND D 1 4 Z 0 1 0),
    unimpl_opcode!(0xA3 AND E 1 4 Z 0 1 0),
    unimpl_opcode!(0xA4 AND H 1 4 Z 0 1 0),
    unimpl_opcode!(0xA5 AND L 1 4 Z 0 1 0),
    unimpl_opcode!(0xA6 AND (HL) 1 8 Z 0 1 0),
    unimpl_opcode!(0xA7 AND A 1 4 Z 0 1 0),
    unimpl_opcode!(0xA8 XOR B 1 4 Z 0 0 0),
    unimpl_opcode!(0xA9 XOR C 1 4 Z 0 0 0),
    unimpl_opcode!(0xAA XOR D 14 Z  0 0 0),
    unimpl_opcode!(0xAB XOR E 1 4 Z 0 0 0),
    unimpl_opcode!(0xAC XOR H 1 4 Z 0 0 0),
    unimpl_opcode!(0xAD XOR L 1 4 Z 0 0 0),
    unimpl_opcode!(0xAE XOR (HL) 1 8 Z 0 0 0),
    ("XOR a", xor::xor_a),
    // 0xBx
    unimpl_opcode!(0xB0 OR B14Z 0 0 0),
    unimpl_opcode!(0xB1 OR C14Z 0 0 0),
    unimpl_opcode!(0xB2 OR D14Z 0 0 0),
    unimpl_opcode!(0xB3 OR E14Z 0 0 0),
    unimpl_opcode!(0xB4 OR H14Z 0 0 0),
    unimpl_opcode!(0xB5 OR L14Z 0 0 0),
    unimpl_opcode!(0xB6 OR (HL)18Z 0 0 0),
    unimpl_opcode!(0xB7 OR A14Z 0 0 0),
    unimpl_opcode!(0xB8 CP B14Z 1 H C),
    unimpl_opcode!(0xB9 CP C14Z 1 H C),
    unimpl_opcode!(0xBA CP D14Z 1 H C),
    unimpl_opcode!(0xBB CP E14Z 1 H C),
    unimpl_opcode!(0xBC CP H14Z 1 H C),
    unimpl_opcode!(0xBD CP L14Z 1 H C),
    unimpl_opcode!(0xBE CP (HL)18Z 1 H C),
    unimpl_opcode!(0xBF CP A14Z 1 H C),
    // 0xCx
    unimpl_opcode!(0xC0 RET NZ 1 20/8 - - - -),
    ("0xC1 POP BC", misc::pop_bc),
    unimpl_opcode!(0xC2 JP NZ,a16 3 16/12 - - - -),
    ("JP a16", jumps::jp_a16),
    unimpl_opcode!(0xC4 CALL NZ,a16 3 24/12 - - - -),
    ("0xC5 PUSH BC", misc::push_bc),
    unimpl_opcode!(0xC6 ADD A,d8 2 8 Z 0 H C),
    unimpl_opcode!(0xC7 RST 00H 1 16 - - - -),
    unimpl_opcode!(0xC8 RET Z 1 20/8 - - - -),
    unimpl_opcode!(0xC9 RET 1 16 - - - -),
    unimpl_opcode!(0xCA JP Z,a16 3 16/12 - - - -),
    ("CB", extended::execute),
    ("CALL z, a16", jumps::call_z_a16),
    ("CALL a16", jumps::call_a16),
    ("ADC a, d8", add::adc_a_d8),
    unimpl_opcode!(0xCF RST 08H 1 16 - - - -),
    // 0xDx
    unimpl_opcode!(0xD0 RET NC 1 20/8 - - - -),
    unimpl_opcode!(0xD1 POP DE 1 12 - - - -),
    unimpl_opcode!(0xD2 JP NC,a16 3 16/12 - - - -),
    unimpl_opcode!(0xD3),
    unimpl_opcode!(0xD4 CALL NC,a16 3 24/12 - - - -),
    unimpl_opcode!(0xD5 PUSH DE 1 16 - - - -),
    unimpl_opcode!(0xD6 SUB d8 28 Z 1 H C),
    unimpl_opcode!(0xD7 RST 10H 1 16 - - - -),
    unimpl_opcode!(0xD8 RET C 1 20/8 - - - -),
    unimpl_opcode!(0xD9 RETI 1 16 - - - -),
    unimpl_opcode!(0xDA JP C,a16 3 16/12 - - - -),
    unimpl_opcode!(0xDB),
    unimpl_opcode!(0xDC CALL C,a16 3 24/12 - - - -),
    unimpl_opcode!(0xDD),
    unimpl_opcode!(0xDE SBC A,d8 2 8 Z 1 H C),
    unimpl_opcode!(0xDF RST 18H 1 16 - - - -),
    // 0xEx
    ("LDH (a8), a", loads::ldh_ptr_a8_a),
    unimpl_opcode!(0xE1 POP HL112- - - -),
    ("0xE2 LD (C),A", loads::ld_ptr_c_a), // actually 1 byte size https://stackoverflow.com/a/41422692
    unimpl_opcode!(0xE3),
    unimpl_opcode!(0xE4),
    unimpl_opcode!(0xE5 PUSH HL116- - - -),
    unimpl_opcode!(0xE6 AND d828Z 0 1 0),
    unimpl_opcode!(0xE7 RST 20H116- - - -),
    unimpl_opcode!(0xE8 ADD SP,r82160 0 H C),
    unimpl_opcode!(0xE9 JP (HL)14- - - -),
    ("LD (a16), a", loads::ld_ptr_a16_a),
    unimpl_opcode!(0xEB),
    unimpl_opcode!(0xEC),
    unimpl_opcode!(0xED),
    unimpl_opcode!(0xEE XOR d828Z 0 0 0),
    unimpl_opcode!(0xEF RST 28H116- - - -),
    // 0xFx
    ("LDH a, (a8)", loads::ldh_a_ptr_a8),
    unimpl_opcode!(0xF1 POP AF 1 12 Z N H C),
    unimpl_opcode!(0xF2 LD A,(C) 1 8 - - - -), // actually 1 byte size https://stackoverflow.com/a/41422692
    ("di", di),
    unimpl_opcode!(0xF4),
    unimpl_opcode!(0xF5 PUSH AF 1 16 - - - -),
    unimpl_opcode!(0xF6 OR d8 2 8 Z 0 0 0),
    unimpl_opcode!(0xF7 RST 30H 1 16 - - - -),
    unimpl_opcode!(0xF8 LD HL,SP+r8 2 12 0 0 H C),
    unimpl_opcode!(0xF9 LD SP,HL 1 8 - - - -),
    unimpl_opcode!(0xFA LD A,(a16) 3 16 - - - -),
    unimpl_opcode!(0xFB EI 1 4 - - - -),
    unimpl_opcode!(0xFC),
    unimpl_opcode!(0xFD),
    ("CP d8", cmp::cp_d8),
    ("RST 38", jumps::rst_38),
];

fn di(memory: &mut Memory, cpu: &mut Cpu) {
    // 0xF3 DI 1 4 - - - -
    cpu.increment_program_counter();
    cpu.clock_cycles(4);

    memory.write_byte(crate::memory::INTERRUPT_ADDRESS, 0);
}
