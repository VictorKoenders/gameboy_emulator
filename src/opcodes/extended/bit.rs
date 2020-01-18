use crate::{Cpu, Memory};

pub fn bit_7_h(_memory: &mut Memory, cpu: &mut Cpu) {
    // 0x7C BIT 7, H 1 4 z 0 1 -
    cpu.increment_program_counter();
    cpu.clock_cycles(4);

    cpu.flags.update_zero(bit_cleared(cpu.h(), 7));
    cpu.flags.clear_subtract();
    cpu.flags.set_half_carry();
}

macro_rules! impl_res {
    ($($code:tt $name:ident $bit:tt $field:tt $set_field:tt),* $(,)?) => {
        $(
            pub fn $name(_: &mut Memory, cpu: &mut Cpu) {
                // $code RES $bit $field 1 4 - - - -
                cpu.increment_program_counter();
                cpu.clock_cycles(4);

                let mut val = cpu.$field();
                val &= !(1 << $bit);
                cpu.$set_field(val);
            }
        )*
    }
}

impl_res! {
    0x87 res_0_a 0 a set_a,
}

fn bit_cleared(value: u8, offset: i8) -> bool {
    !bit_set(value, offset)
}

fn bit_set(value: u8, offset: i8) -> bool {
    (value >> offset) & 1 == 1
}
