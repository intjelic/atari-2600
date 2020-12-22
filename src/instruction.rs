// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of Atari 2600 Emulator which is released under the
// MIT license. Please refer to the LICENSE file that can be found at the root
// of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, September 2020

//! The MOS 6507 instructions.
//!
//! This module contains the implementation of the 6507 instructions. Each
//! instruction takes a number of cycle to execute which is returned by the
//! function. The cycle number is also varying according to the value of the
//! operand.
//!
//! The instructions:
//!
//! - ADC, AND, ASL
//! - BCC, BCS, BEQ, BIT, BMI, BNE, BPL, BRK, BVC, BVS
//! - CLC, CLD, CLI, CLV, CMP, CPX, CPY
//! - DEC, DEX, DEY
//! - EOR
//! - INC, INX, INY
//! - JMP, JSR
//! - LDA, LDX, LDY
//! - NOP
//! - ORA
//! - PHA, PHP, PLA, PLP
//! - ROL, ROR, RTI, RTS
//! - SBC, SEC, SED, SEI, STA, STX, STY
//! - TAX, TAY, TSX, TXA, TXS, TYA
//!
//! TODO; Mark instructions that were excluded.
//!
//! Note that they're tightly coupled with the **Console** struct. In fact,
//! they were put outside just to increase readability.
//!
use super::console::Console;
use super::addressing_mode::*;

/// Increment a byte value by one.
///
/// This function increments a byte value by one. If it overflows, its value
/// becomes 0.
///
fn increment_byte(value: &mut u8) {
    *value = value.wrapping_add(1);
}

/// Decrement a byte value by one.
///
/// This function decrements a byte value by one. If it underflows, its value
/// becomes 255.
///
fn decrement_byte(value: &mut u8) {
    *value = value.wrapping_sub(1);
}

/// Update the zero and negative flags.
///
/// This function updates the zero and negative flags according to a value. If
/// the value is 0, it raises the zero flag. If the value when interpreted as
/// signed is negative (when first bit is 1), it raises the negative flag.
///
fn update_zero_and_negative_flags(value: &u8, zero_flag: &mut bool, negative_flag: &mut bool) {
    *zero_flag = *value == 0;
    *negative_flag = *value > 127;
}

/// Brief description.
///
/// Long description.
///
fn transfer_byte(source: &mut u8, destination: &mut u8) {
    *destination = *source;
}

/// Shift the bits of a byte to the left.
///
/// This function takes an input bit (which is either 0 or 1) to shift the value
/// with, and it takes an output bit which will be updated with the discarded
/// bit.
///
fn shift_left(value: &mut u8, bit_in: bool, bit_out: &mut bool) {
    *bit_out = *value & 0b10000000 > 0;
    *value <<= 1;
    if bit_in {
        *value |= 0b00000001;
    }
}

/// Shift the bits of a byte to the right.
///
/// This function takes an input bit (which is either 0 or 1) to shift the value
/// with, and it takes an output bit which will be updated with the discarded
/// bit.
///
fn shift_right(value: &mut u8, bit_in: bool, bit_out: &mut bool) {
    *bit_out = *value & 0b00000001 > 0;
    *value >>= 1;
    if bit_in {
        *value |= 0b10000000;
    }
}

/// The ADC instruction.
///
/// This instruction makes an addition with the accumulator, the operand and
/// the value of the carry flag (0 or 1), and store it in the accumulator. If
/// an overflow occurred, the carry flag is set to 1, otherwise it's set to 0.
/// It also updates the zero and negative flags according to the final value.
///
/// TODO; The documentation says the overflow flag is updated, but I'm unable
/// to understand in which context.
///
pub fn adc_instruction(console: &mut Console, opcode: u8) -> u32 {
    let (index, cycles) = match opcode {
        0x_69 => (immediate(console), 2),
        0x_65 => (zero_page(console), 3),
        0x_75 => (zero_page_x(console), 4),
        0x_6D => (absolute(console), 4),
        0x_7D => {
            match absolute_x(console) {
                (index, false) => (index, 4),
                (index, true) => (index, 5)
            }
        },
        0x_79 => {
            match absolute_y(console) {
                (index, false) => (index, 4),
                (index, true) => (index, 5)
            }
        },
        0x_61 => (indexed_indirect(console), 6),
        0x_71 => {
            match indirect_indexed(console) {
                (index, false) => (index, 5),
                (index, true) => (index, 6)
            }
        },
        _ => panic!("opcode {:#X} not associated to ADC instruction", opcode)
    };

    let value = *console.memory_mut(index);

    // The operation is A + M + 1, and thus, it can overflow during either of
    // the two additions. We make sure to intercept if it's overflowing in both
    // addition and update the cary flag accordingly.
    let (new_value, has_overflowed_a) = console.accumulator.overflowing_add(value);
    let (new_value, has_overflowed_b) = if console.carry_flag {
        new_value.overflowing_add(1)
    } else {
        (new_value, false)
    };

    console.accumulator = new_value;
    console.carry_flag = has_overflowed_a || has_overflowed_b;

    update_zero_and_negative_flags(
        &console.accumulator,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    // TODO; This flag is documented as potentially modified, but in which context ?
    // console.overflow_flag = true;

    cycles
}

/// The AND instruction.
///
/// This instruction performs a bitwise AND operation with the operand and the
/// accumulator, then stores the result in the accumulator. It also updates the
/// zero and negative flags according to the resulting value.
///
pub fn and_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (index, cycles) = match opcode {
        0x_29 => (immediate(console), 2),
        0x_25 => (zero_page(console), 3),
        0x_35 => (zero_page_x(console), 4),

        0x_2D => (absolute(console), 4),
        0x_3D => {
            match absolute_x(console) {
                (index, false) => (index, 4),
                (index, true) => (index, 5)
            }
        },
        0x_39 => {
            match absolute_y(console) {
                (index, false) => (index, 4),
                (index, true) => (index, 5)
            }
        },
        0x_21 => (indexed_indirect(console), 6),
        0x_31 => {
            match indirect_indexed(console) {
                (index, false) => (index, 5),
                (index, true) => (index, 6)
            }
        },
        _ => panic!("opcode {:#X} not associated to AND instruction", opcode)
    };

    let value = console.memory_mut(index);
    console.accumulator = *value & console.accumulator;

    update_zero_and_negative_flags(
        &console.accumulator,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    cycles
}

/// The ASL instruction.
///
/// Long description.
///
/// Unlike the ROL instruction, it doesn't shift the value with a the carry flag.
///
pub fn asl_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (operand, cycles) = match opcode {
        0x_0A => (&mut console.accumulator, 2),
        _ => {
            let (index, cycles) = match opcode {
                0x_06 => (zero_page(console),    5),
                0x_16 => (zero_page_x(console),  6),
                0x_0E => (absolute(console),     6),
                0x_1E => (absolute_x(console).0, 7),
                _ => panic!("opcode {:#X} not associated to ASL instruction", opcode)
            };

            (console.memory_mut(index), cycles)
        }
    };

    shift_left(operand, false, &mut console.carry_flag);

    update_zero_and_negative_flags(
        operand,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    cycles
}

/// The BCC instruction.
///
/// Long description.
///
pub fn bcc_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (operand, mut cycles) = match opcode {
        0x_90 => (relative(console), 2),
        _ => panic!("opcode {:#X} not associated to BCC instruction", opcode)
    };

    if console.carry_flag == false {
        let page = console.pointer_counter.to_be_bytes()[0];

        if operand > 0 {
            console.pointer_counter = console.pointer_counter.wrapping_add(operand as u16);
        }
        else {
            let value = !(operand as u8) + 1;
            console.pointer_counter = console.pointer_counter.wrapping_sub(value as u16);
        }

        // Branch is occuring, increment the cycle count by one if on the same
        // page, by two if on a different page.
        if console.pointer_counter.to_be_bytes()[0] == page {
            cycles += 1;
        } else {
            cycles += 2;
        }
    }

    cycles
}

/// The BCS instruction.
///
/// Long description.
///
pub fn bcs_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (operand, mut cycles) = match opcode {
        0x_B0 => (relative(console), 2),
        _ => panic!("opcode {:#X} not associated to BCS instruction", opcode)
    };

    if console.carry_flag == true {
        let page = console.pointer_counter.to_be_bytes()[0];

        if operand > 0 {
            console.pointer_counter = console.pointer_counter.wrapping_add(operand as u16);
        }
        else {
            let value = !(operand as u8) + 1;
            console.pointer_counter = console.pointer_counter.wrapping_sub(value as u16);
        }

        // Branch is occuring, increment the cycle count by one if on the same
        // page, by two if on a different page.
        if console.pointer_counter.to_be_bytes()[0] == page {
            cycles += 1;
        } else {
            cycles += 2;
        }
    }

    cycles
}

/// The BEQ instruction.
///
/// Long description.
///
pub fn beq_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (operand, mut cycles) = match opcode {
        0x_F0 => (relative(console), 2),
        _ => panic!("opcode {:#X} not associated to BEQ instruction", opcode)
    };

    if console.zero_flag == true {
        let page = console.pointer_counter.to_be_bytes()[0];

        if operand > 0 {
            console.pointer_counter = console.pointer_counter.wrapping_add(operand as u16);
        }
        else {
            let value = !(operand as u8) + 1;
            console.pointer_counter = console.pointer_counter.wrapping_sub(value as u16);
        }

        // Branch is occuring, increment the cycle count by one if on the same
        // page, by two if on a different page.
        if console.pointer_counter.to_be_bytes()[0] == page {
            cycles += 1;
        } else {
            cycles += 2;
        }
    }

    cycles
}

/// The BIT instruction.
///
/// Long description.
///
pub fn bit_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (index, cycles) = match opcode {
        0x_24 => (zero_page(console), 3),
        0x_2C => (absolute(console), 4),
        _ => panic!("opcode {:#X} not associated to BIT instruction", opcode)
    };

    let operand = console.memory_mut(index);

    let bit_7 = *operand & 0b1000_0000 > 0;
    let bit_6 = *operand & 0b0100_0000 > 0;

    console.negative_flag = bit_7;
    console.overflow_flag = bit_6;

    console.zero_flag = console.accumulator & *operand == 0;

    cycles
}

/// The BMI instruction.
///
/// Long description.
///
pub fn bmi_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (operand, mut cycles) = match opcode {
        0x_30 => (relative(console), 2),
        _ => panic!("opcode {:#X} not associated to BMI instruction", opcode)
    };

    if console.negative_flag == true {
        let page = console.pointer_counter.to_be_bytes()[0];

        if operand > 0 {
            console.pointer_counter = console.pointer_counter.wrapping_add(operand as u16);
        }
        else {
            let value = !(operand as u8) + 1;
            console.pointer_counter = console.pointer_counter.wrapping_sub(value as u16);
        }

        // Branch is occurring, increment the cycle count by one if on the same
        // page, by two if on a different page.
        if console.pointer_counter.to_be_bytes()[0] == page {
            cycles += 1;
        } else {
            cycles += 2;
        }
    }

    cycles
}

/// The BNE instruction.
///
/// Long description.
///
pub fn bne_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (operand, mut cycles) = match opcode {
        0x_D0 => (relative(console), 2),
        _ => panic!("opcode {:#X} not associated to BNE instruction", opcode)
    };

    if console.zero_flag == false {
        let page = console.pointer_counter.to_be_bytes()[0];

        if operand > 0 {
            console.pointer_counter = console.pointer_counter.wrapping_add(operand as u16);
        }
        else {
            let value = !(operand as u8) + 1;
            console.pointer_counter = console.pointer_counter.wrapping_sub(value as u16);
        }

        // Branch is occurring, increment the cycle count by one if on the same
        // page, by two if on a different page.
        if console.pointer_counter.to_be_bytes()[0] == page {
            cycles += 1;
        } else {
            cycles += 2;
        }
    }

    cycles
}

/// The BPL instruction.
///
/// Long description.
///
pub fn bpl_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (operand, mut cycles) = match opcode {
        0x_10 => (relative(console), 2),
        _ => panic!("opcode {:#X} not associated to BPL instruction", opcode)
    };

    if console.negative_flag == false {
        let page = console.pointer_counter.to_be_bytes()[0];

        if operand > 0 {
            console.pointer_counter = console.pointer_counter.wrapping_add(operand as u16);
        }
        else {
            let value = !(operand as u8) + 1;
            console.pointer_counter = console.pointer_counter.wrapping_sub(value as u16);
        }

        // Branch is occurring, increment the cycle count by one if on the same
        // page, by two if on a different page.
        if console.pointer_counter.to_be_bytes()[0] == page {
            cycles += 1;
        } else {
            cycles += 2;
        }
    }

    cycles
}

/// The BRK instruction.
///
/// Long description.
///
pub fn brk_instruction(_console: &mut Console, _opcode: u8) -> u32 {
    // TODO; To be implemented.

    0
}

/// The BVC instruction.
///
/// Long description.
///
pub fn bvc_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (operand, mut cycles) = match opcode {
        0x_50 => (relative(console), 2),
        _ => panic!("opcode {:#X} not associated to BVC instruction", opcode)
    };

    if console.overflow_flag == false {
        let page = console.pointer_counter.to_be_bytes()[0];

        if operand > 0 {
            console.pointer_counter = console.pointer_counter.wrapping_add(operand as u16);
        }
        else {
            let value = !(operand as u8) + 1;
            console.pointer_counter = console.pointer_counter.wrapping_sub(value as u16);
        }

        // Branch is occurring, increment the cycle count by one if on the same
        // page, by two if on a different page.
        if console.pointer_counter.to_be_bytes()[0] == page {
            cycles += 1;
        } else {
            cycles += 2;
        }
    }

    cycles
}

/// The BVS instruction.
///
/// Long description.
///
pub fn bvs_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (operand, mut cycles) = match opcode {
        0x_70 => (relative(console), 2),
        _ => panic!("opcode {:#X} not associated to BVS instruction", opcode)
    };

    if console.overflow_flag == true {
        let page = console.pointer_counter.to_be_bytes()[0];

        if operand > 0 {
            console.pointer_counter = console.pointer_counter.wrapping_add(operand as u16);
        }
        else {
            let value = !(operand as u8) + 1;
            console.pointer_counter = console.pointer_counter.wrapping_sub(value as u16);
        }

        // Branch is occuring, increment the cycle count by one if on the same
        // page, by two if on a different page.
        if console.pointer_counter.to_be_bytes()[0] == page {
            cycles += 1;
        } else {
            cycles += 2;
        }
    }

    cycles
}

/// The CLC instruction.
///
/// This instruction does something.
///
pub fn clc_instruction(console: &mut Console, opcode: u8) -> u32 {
    assert_eq!(opcode, 0x_18, "opcode {:#X} not associated to CLC instruction", opcode);
    console.carry_flag = false;

    2
}

/// The CLD instruction.
///
/// This instruction does something.
///
pub fn cld_instruction(console: &mut Console, opcode: u8) -> u32 {
    assert_eq!(opcode, 0x_D8, "opcode {:#X} not associated to CLD instruction", opcode);
    console.decimal_flag = false;

    2
}

/// The CLI instruction.
///
/// This instruction does something.
///
pub fn cli_instruction(console: &mut Console, opcode: u8) -> u32 {
    assert_eq!(opcode, 0x_58, "opcode {:#X} not associated to CLI instruction", opcode);
    console.interrupt_flag = false;

    2
}

/// The CLV instruction.
///
/// This instruction does something.
///
pub fn clv_instruction(console: &mut Console, opcode: u8) -> u32 {
    assert_eq!(opcode, 0x_B8, "opcode {:#X} not associated to CLV instruction", opcode);
    console.overflow_flag = false;

    2
}

/// The CMP instruction.
///
/// This instruction does something.
///
pub fn cmp_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (index, cycles) = match opcode {
        0x_C9 => (immediate(console), 2),
        0x_C5 => (zero_page(console), 3),
        0x_D5 => (zero_page_x(console), 4),

        0x_CD => (absolute(console), 4),
        0x_DD => {
            match absolute_x(console) {
                (index, false) => (index, 4),
                (index, true) => (index, 5)
            }
        },
        0x_D9 => {
            match absolute_y(console) {
                (index, false) => (index, 4),
                (index, true) => (index, 5)
            }
        },
        0x_C1 => (indexed_indirect(console), 6),
        0x_D1 => {
            match indirect_indexed(console) {
                (index, false) => (index, 5),
                (index, true) => (index, 6)
            }
        },
        _ => panic!("opcode {:#X} not associated to CMP instruction", opcode)
    };

    // Update the carry flag according to A >= M.
    let value = console.memory(index);
    console.carry_flag = if console.accumulator >= *value { true } else { false };

    // Update the zero and negative flag according to X - M.
    update_zero_and_negative_flags(
        &console.accumulator.wrapping_sub(*value),
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    cycles
}

/// The CPX instruction.
///
/// This instruction does something.
///
pub fn cpx_instruction(console: &mut Console, opcode: u8) -> u32 {
    let (index, cycles) = match opcode {
        0x_E0 => (immediate(console), 2),
        0x_E4 => (zero_page(console), 3),
        0x_EC => (absolute(console), 4),
        _ => panic!("opcode {:#X} not associated to CPX instruction", opcode)
    };

    // Update the carry flag according to X >= M.
    let value = console.memory(index);
    console.carry_flag = if console.x_register >= *value { true } else { false };

    // Update the zero and negative flag according to X - M.
    update_zero_and_negative_flags(
        &console.x_register.wrapping_sub(*value),
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    cycles
}

/// The CPY instruction.
///
/// This instruction does something.
///
pub fn cpy_instruction(console: &mut Console, opcode: u8) -> u32 {
    let (index, cycles) = match opcode {
        0x_C0 => (immediate(console), 2),
        0x_C4 => (zero_page(console), 3),
        0x_CC => (absolute(console), 4),
        _ => panic!("opcode {:#X} not associated to CPY instruction", opcode)
    };

    // Update the carry flag according to Y >= M.
    let value = console.memory(index);
    console.carry_flag = if console.y_register >= *value { true } else { false };

    // Update the zero and negative flag according to Y - M.
    update_zero_and_negative_flags(
        &console.y_register.wrapping_sub(*value),
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    cycles
}

/// The DEC instruction.
///
/// This instruction does something.
///
pub fn dec_instruction(console: &mut Console, opcode: u8) -> u32 {
    let (index, cycles) = match opcode {
        0x_C6 => (zero_page(console), 5),
        0x_D6 => (zero_page_x(console), 6),
        0x_CE => (absolute(console), 6),
        0x_DE => (absolute_x(console).0, 7),
        _ => panic!("opcode {} not associated to DEC instruction", opcode)
    };

    let value = console.memory_mut(index);

    decrement_byte(value);
    update_zero_and_negative_flags(
        value,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    cycles
}

/// The DEX instruction.
///
/// This instruction decrements the X register by one. It also updates the zero
/// and negative flags.
///
pub fn dex_instruction(console: &mut Console, opcode: u8) -> u32 {
    assert_eq!(opcode, 0x_CA, "opcode {:#X} not associated to DEX instruction", opcode);

    decrement_byte(&mut console.x_register);
    update_zero_and_negative_flags(
        &mut console.x_register,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    2
}

/// The DEY instruction.
///
/// This instruction decrements the Y register by one. It also updates the zero
/// and negative flags.
///
pub fn dey_instruction(console: &mut Console, opcode: u8) -> u32 {
    assert_eq!(opcode, 0x_88, "opcode {:#X} not associated to DEY instruction", opcode);

    decrement_byte(&mut console.y_register);
    update_zero_and_negative_flags(
        &mut console.y_register,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    2
}

/// The EOR instruction.
///
/// Long description.
///
pub fn eor_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (index, cycles) = match opcode {
        0x_49 => (immediate(console), 2),
        0x_45 => (zero_page(console), 3),
        0x_55 => (zero_page_x(console), 4),

        0x_4D => (absolute(console), 4),
        0x_5D => {
            match absolute_x(console) {
                (index, false) => (index, 4),
                (index, true) => (index, 5)
            }
        },
        0x_59 => {
            match absolute_y(console) {
                (index, false) => (index, 4),
                (index, true) => (index, 5)
            }
        },
        0x_41 => (indexed_indirect(console), 6),
        0x_51 => {
            match indirect_indexed(console) {
                (index, false) => (index, 5),
                (index, true) => (index, 6)
            }
        },
        _ => panic!("opcode {:#X} not associated to EOR instruction", opcode)
    };

    let value = console.memory(index);
    console.accumulator ^= *value;

    update_zero_and_negative_flags(
        &console.accumulator,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    cycles
}

/// The INC instruction.
///
/// This instruction does something.
///
pub fn inc_instruction(console: &mut Console, opcode: u8) -> u32 {
    let (index, cycles) = match opcode {
        0x_E6 => (zero_page(console), 5),
        0x_F6 => (zero_page_x(console), 6),
        0x_EE => (absolute(console), 6),
        0x_FE => (absolute_x(console).0, 7),
        _ => panic!("opcode {} not associated to INC instruction", opcode)
    };

    let value = console.memory_mut(index);

    increment_byte(value);
    update_zero_and_negative_flags(
        value,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    cycles
}

/// The INX instruction.
///
/// This instruction increments the X register by one. It also updates the zero
/// and negative flags.
///
pub fn inx_instruction(console: &mut Console, opcode: u8) -> u32 {
    assert_eq!(opcode, 0x_E8, "opcode {:#X} not associated to INX instruction", opcode);

    increment_byte(&mut console.x_register);
    update_zero_and_negative_flags(
        &mut console.x_register,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    2
}

/// The INY instruction.
///
/// This instruction increments the Y register by one. It also updates the zero
/// and negative flags.
///
pub fn iny_instruction(console: &mut Console, opcode: u8) -> u32 {
    assert_eq!(opcode, 0x_C8, "opcode {:#X} not associated to INY instruction", opcode);

    increment_byte(&mut console.y_register);
    update_zero_and_negative_flags(
        &mut console.y_register,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    2
}

/// The JMP instruction.
///
/// Long description.
///
pub fn jmp_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (pointer_counter, cycles) = match opcode {
        0x_4C => (absolute(console), 3),
        0x_6C => {
            // Note that advancing the pointer here is irrelevant as the pointer
            // counter is modified later.
            let indirect_index = absolute(console);

            let ll = *console.memory(indirect_index);
            let hh = *console.memory(indirect_index + 1);

            (u16::from_le_bytes([ll, hh]), 5)
        },
        _ => panic!("opcode {} not associated to JMP instruction", opcode)
    };

    console.pointer_counter = pointer_counter;

    cycles
}

/// The JSR instruction.
///
/// Long description.
///
pub fn jsr_instruction(console: &mut Console, opcode: u8) -> u32 {

    assert_eq!(opcode, 0x_20, "opcode {:#X} not associated to JSR instruction", opcode);

    let pointer_counter = absolute(console);

    // let [ll, hh] = console.pointer_counter.to_le_bytes();
    let [ll, hh] = (console.pointer_counter - 1).to_le_bytes(); // that doesn't
    // seem right, but the online emulator seems to do that way
    console.push_value(hh);
    console.push_value(ll);

    console.pointer_counter = pointer_counter;

    6
}


/// The LDA instruction.
///
/// Long description.
///
pub fn lda_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (index, cycles) = match opcode {
        0x_A9 => (immediate(console), 2),
        0x_A5 => (zero_page(console), 3),
        0x_B5 => (zero_page_x(console), 4),

        0x_AD => (absolute(console), 4),
        0x_BD => {
            match absolute_x(console) {
                (index, false) => (index, 4),
                (index, true) => (index, 5)
            }
        },
        0x_B9 => {
            match absolute_y(console) {
                (index, false) => (index, 4),
                (index, true) => (index, 5)
            }
        },
        0x_A1 => (indexed_indirect(console), 6),
        0x_B1 => {
            match indirect_indexed(console) {
                (index, false) => (index, 5),
                (index, true) => (index, 6)
            }
        },
        _ => panic!("opcode {:#X} not associated to LDA instruction", opcode)
    };

    let value = console.memory(index);
    console.accumulator = *value;

    update_zero_and_negative_flags(
        &console.accumulator,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    cycles
}

/// The LDX instruction.
///
/// Long description.
///
pub fn ldx_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (index, cycles) = match opcode {
        0x_A2 => (immediate(console), 2),
        0x_A6 => (zero_page(console),3),
        0x_B6 => (zero_page_y(console), 4),
        0x_AE => (absolute(console), 4),
        0x_BE => {
            match absolute_y(console) {
                (index, false) => (index, 4),
                (index, true) => (index, 5)
            }
        }
        _ => panic!("opcode {} not associated to LDX instruction", opcode)
    };

    console.x_register = *console.memory(index);
    update_zero_and_negative_flags(
        &console.x_register,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    cycles
}

/// The LDY instruction.
///
/// Long description.
///
pub fn ldy_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (index, cycles) = match opcode {
        0x_A0 => (immediate(console), 2),
        0x_A4 => (zero_page(console),3),
        0x_B4 => (zero_page_x(console), 4),
        0x_AC => (absolute(console), 4),
        0x_BC => {
            match absolute_x(console) {
                (index, false) => (index, 4),
                (index, true) => (index, 5)
            }
        }
        _ => panic!("opcode {} not associated to LDY instruction", opcode)
    };

    console.y_register = *console.memory(index);
    update_zero_and_negative_flags(
        &console.y_register,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    cycles
}

/// The LSR instruction.
///
/// Long description.
///
pub fn lsr_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (operand, cycles) = match opcode {
        0x_4A => (&mut console.accumulator, 2),
        _ => {
            let (index, cycles) = match opcode {
                0x_46 => (zero_page(console),    5),
                0x_56 => (zero_page_x(console),  6),
                0x_4E => (absolute(console),     6),
                0x_5E => (absolute_x(console).0, 7),
                _ => panic!("opcode {:#X} not associated to LSR instruction", opcode)
            };

            (console.memory_mut(index), cycles)
        }
    };

    shift_right(operand, false, &mut console.carry_flag);

    // Note that while the zero flag must always be set to 0, this function will
    // always update it correctly since the entering bit was 0.
    update_zero_and_negative_flags(
        operand,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    cycles
}

/// The NOP instruction.
///
/// Long description.
///
pub fn nop_instruction(_console: &mut Console, opcode: u8) -> u32 {

    assert_eq!(opcode, 0x_EA, "opcode {:#X} not associated to ORA instruction", opcode);

    // Absolutely nothing to do. The pointer counter is advanced by the caller.

    2
}

/// The ORA instruction.
///
/// Long description.
///
pub fn ora_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (index, cycles) = match opcode {
        0x_09 => (immediate(console), 2),
        0x_05 => (zero_page(console), 3),
        0x_15 => (zero_page_x(console), 4),

        0x_0D => (absolute(console), 4),
        0x_1D => {
            match absolute_x(console) {
                (index, false) => (index, 4),
                (index, true) => (index, 5)
            }
        },
        0x_19 => {
            match absolute_y(console) {
                (index, false) => (index, 4),
                (index, true) => (index, 5)
            }
        },
        0x_01 => (indexed_indirect(console), 6),
        0x_11 => {
            match indirect_indexed(console) {
                (index, false) => (index, 5),
                (index, true) => (index, 6)
            }
        },
        _ => panic!("opcode {:#X} not associated to ORA instruction", opcode)
    };

    let value = console.memory(index);
    console.accumulator |= *value;

    update_zero_and_negative_flags(
        &console.accumulator,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    cycles
}

/// The PHA instruction.
///
/// Long description.
///
pub fn pha_instruction(console: &mut Console, opcode: u8) -> u32 {

    assert_eq!(opcode, 0x_48, "opcode {:#X} not associated to PHA instruction", opcode);
    console.push_value(console.accumulator);

    3
}

/// The PHP instruction.
///
/// Long description.
///
pub fn php_instruction(console: &mut Console, opcode: u8) -> u32 {

    assert_eq!(opcode, 0x_08, "opcode {:#X} not associated to PHP instruction", opcode);

    let mut status_flag = 0b0000_0000u8;
    if console.negative_flag  { status_flag |= 0b1000_0000 };
    if console.overflow_flag  { status_flag |= 0b0100_0000 };
    if console.break_flag     { status_flag |= 0b0001_0000 };
    if console.decimal_flag   { status_flag |= 0b0000_1000 };
    if console.interrupt_flag { status_flag |= 0b0000_0100 };
    if console.zero_flag      { status_flag |= 0b0000_0010 };
    if console.carry_flag     { status_flag |= 0b0000_0001 };

    console.push_value(status_flag);

    3
}

/// The PLA instruction.
///
/// Long description.
///
pub fn pla_instruction(console: &mut Console, opcode: u8) -> u32 {

    assert_eq!(opcode, 0x_68, "opcode {:#X} not associated to PLA instruction", opcode);
    console.accumulator = console.pop_value();

    4
}

/// The PLP instruction.
///
/// Long description.
///
pub fn plp_instruction(console: &mut Console, opcode: u8) -> u32 {

    assert_eq!(opcode, 0x_28, "opcode {:#X} not associated to PLP instruction", opcode);

    let status_flag = console.pop_value();
    console.negative_flag  = status_flag & 0b1000_0000 > 0;
    console.overflow_flag  = status_flag & 0b0100_0000 > 0;
    console.break_flag     = status_flag & 0b0001_0000 > 0;
    console.decimal_flag   = status_flag & 0b0000_1000 > 0;
    console.interrupt_flag = status_flag & 0b0000_0100 > 0;
    console.zero_flag      = status_flag & 0b0000_0010 > 0;
    console.carry_flag     = status_flag & 0b0000_0001 > 0;

    4
}

/// The ROL instruction.
///
/// Long description.
///
pub fn rol_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (operand, cycles) = match opcode {
        0x_2A => (&mut console.accumulator, 2),
        _ => {
            let (index, cycles) = match opcode {
                0x_26 => (zero_page(console), 5),
                0x_36 => (zero_page_x(console), 6),
                0x_2E => (absolute(console), 6),
                0x_3E => (absolute_x(console).0, 7),
                _ => panic!("opcode {:#X} not associated to ROL instruction", opcode)
            };

            (console.memory_mut(index), cycles)
        }
    };

    shift_left(operand, console.carry_flag, &mut console.carry_flag);
    update_zero_and_negative_flags(
        operand,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    cycles
}

/// The ROR instruction.
///
/// Long description.
///
pub fn ror_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (operand, cycles) = match opcode {
        0x_6A => (&mut console.accumulator, 2),
        _ => {
            let (index, cycles) = match opcode {
                0x_66 => (zero_page(console), 5),
                0x_76 => (zero_page_x(console), 6),
                0x_6E => (absolute(console), 6),
                0x_7E => (absolute_x(console).0, 7),
                _ => panic!("opcode {:#X} not associated to ROR instruction", opcode)
            };

            (console.memory_mut(index), cycles)
        }
    };

    shift_right(operand, console.carry_flag, &mut console.carry_flag);
    update_zero_and_negative_flags(
        operand,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    cycles
}

/// The RTI instruction.
///
/// Long description.
///
pub fn rti_instruction(_console: &mut Console, _opcode: u8) -> u32 {

    // TODO; Not implemented yet.

    0
}

/// The RTS instruction.
///
/// Long description.
///
pub fn rts_instruction(console: &mut Console, opcode: u8) -> u32 {

    assert_eq!(opcode, 0x_60, "opcode {:#X} not associated to RTS instruction", opcode);

    let ll = console.pop_value();
    let hh = console.pop_value();
    console.pointer_counter = u16::from_le_bytes([ll, hh]) + 1;

    6
}

/// The SBC instruction.
///
/// Long description.
///
pub fn sbc_instruction(_console: &mut Console, _opcode: u8) -> u32 {

    // TODO; Not implemented yet.
    0
}

/// The SEC instruction.
///
/// Long description.
///
pub fn sec_instruction(console: &mut Console, opcode: u8) -> u32 {

    assert_eq!(opcode, 0x_38, "opcode {:#X} not associated to SEC instruction", opcode);
    console.carry_flag = true;

    2
}

/// The SED instruction.
///
/// Long description.
///
pub fn sed_instruction(console: &mut Console, opcode: u8) -> u32 {

    assert_eq!(opcode, 0x_F8, "opcode {:#X} not associated to SED instruction", opcode);
    console.decimal_flag = true;

    2
}

/// The SEI instruction.
///
/// Long description.
///
pub fn sei_instruction(console: &mut Console, opcode: u8) -> u32 {

    assert_eq!(opcode, 0x_78, "opcode {:#X} not associated to SEI instruction", opcode);
    console.interrupt_flag = true;

    2
}

/// The STA instruction.
///
/// Long description.
///
pub fn sta_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (index, cycles) = match opcode {
        0x_85 => (zero_page(console), 3),
        0x_95 => (zero_page_x(console), 4),
        0x_8D => (absolute(console), 4),
        0x_9D => (absolute_x(console).0, 5),
        0x_99 => (absolute_y(console).0, 5),
        0x_81 => (indexed_indirect(console), 6),
        0x_91 => (indirect_indexed(console).0, 6),
        _ => panic!("opcode {:#X} not associated to STA instruction", opcode)
    };

    *console.memory_mut(index) = console.accumulator;

    cycles
}

/// The STX instruction.
///
/// This instruction does something.
///
/// STX	....	store X
///
pub fn stx_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (index, cycles) = match opcode {
        0x_86 => (zero_page(console), 3),
        0x_96 => (zero_page_y(console), 4),
        0x_8E => (absolute(console), 4),
        _ => panic!("opcode {:#X} not associated to STX instruction", opcode)
    };

    *console.memory_mut(index) = console.x_register;

    cycles
}

/// The STY instruction.
///
/// This instruction does something.
///
/// STY	....	store Y
///
pub fn sty_instruction(console: &mut Console, opcode: u8) -> u32 {

    let (index, cycles) = match opcode {
        0x_84 => (zero_page(console), 3),
        0x_94 => (zero_page_x(console), 4),
        0x_8C => (absolute(console), 4),
        _ => panic!("opcode {:#X} not associated to STY instruction", opcode)
    };

    *console.memory_mut(index) = console.y_register;

    cycles
}

/// The TAX instruction.
///
/// This instruction does something.
///
pub fn tax_instruction(console: &mut Console, opcode: u8) -> u32 {
    assert_eq!(opcode, 0x_AA, "opcode {:#X} not associated to TAX instruction", opcode);

    transfer_byte(&mut console.accumulator, &mut console.x_register);
    update_zero_and_negative_flags(
        &mut console.x_register,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    2
}

/// The TAY instruction.
///
/// This instruction does something.
///
/// TAY	....	transfer accumulator to Y
///
///
pub fn tay_instruction(console: &mut Console, opcode: u8) -> u32 {
    assert_eq!(opcode, 0x_A8, "opcode {:#X} not associated to TAY instruction", opcode);

    transfer_byte(&mut console.accumulator, &mut console.y_register);
    update_zero_and_negative_flags(
        &mut console.y_register,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    2
}

/// The TSX instruction.
///
/// This instruction does something.
///
pub fn tsx_instruction(console: &mut Console, opcode: u8) -> u32 {
    assert_eq!(opcode, 0x_BA, "opcode {:#X} not associated to TSX instruction", opcode);

    transfer_byte(&mut console.stack_pointer, &mut console.x_register);
    update_zero_and_negative_flags(
        &mut console.x_register,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    2
}

/// The TXA instruction.
///
/// This instruction does something.
///
/// TXA	....	transfer X to accumulator
///
pub fn txa_instruction(console: &mut Console, opcode: u8) -> u32 {
    assert_eq!(opcode, 0x_8A, "opcode {:#X} not associated to TXA instruction", opcode);

    transfer_byte(&mut console.x_register, &mut console.accumulator);
    update_zero_and_negative_flags(
        &mut console.accumulator,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    2
}

/// The TXS instruction.
///
/// This instruction does something.
///
pub fn txs_instruction(console: &mut Console, opcode: u8) -> u32 {
    assert_eq!(opcode, 0x_9A, "opcode {:#X} not associated to TXS instruction", opcode);
    transfer_byte(&mut console.x_register, &mut console.stack_pointer);

    2
}

/// The TYA instruction.
///
/// This instruction does something.
///
/// TYA	....	transfer Y to accumulator
///
pub fn tya_instruction(console: &mut Console, opcode: u8) -> u32 {
    assert_eq!(opcode, 0x_98, "opcode {:#X} not associated to TYA instruction", opcode);

    transfer_byte(&mut console.y_register, &mut console.accumulator);
    update_zero_and_negative_flags(
        &mut console.accumulator,
        &mut console.zero_flag,
        &mut console.negative_flag,
    );

    2
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cartridge::Cartridge;

    fn setup_instruction(console: &mut Console, bytes: Vec<u8>) {
        // setup_instruction_x(console, bytes, 0x_200);
        setup_instruction_x(console, bytes, 0x_00);
    }

    fn setup_instruction_x(console: &mut Console, bytes: Vec<u8>, index: u16) {
        // todo; replace this code with more idiomatic Rust
        let mut i: u16 = 0;
        for byte in bytes.iter() {
            *console.memory_mut(index + i) = *byte;
            i += 1;
        };

        console.pointer_counter = index;
    }

    fn execute_instruction(console: &mut Console, instruction: fn(&mut Console, u8) -> u32) -> u32 {
        let opcode = *console.pointed_value();
        console.advance_pointer();

        instruction(console, opcode)
    }

    #[test]
    fn test_update_zero_and_negative_flags() {
        // To be implemented.
    }

    #[test]
    fn test_adc_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));

        {
            setup_instruction(&mut console, vec![0x_69, 0x_86]);

            console.accumulator = 0x_43;
            console.carry_flag = true;
            console.zero_flag = true;
            console.negative_flag = false;

            let cycles = execute_instruction(&mut console, adc_instruction);

            assert_eq!(console.accumulator, 0x_CA);
            assert_eq!(console.carry_flag, false);
            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, true);

            assert_eq!(cycles, 2);
        }

        {
            setup_instruction(&mut console, vec![0x_65, 0x_E5]);
            *console.memory_mut(0x_E5) = 0x_D1;

            console.accumulator = 0x_79;
            console.carry_flag = true;
            console.zero_flag = true;
            console.negative_flag = true;

            let cycles = execute_instruction(&mut console, adc_instruction);

            assert_eq!(console.accumulator, 0x_4B);
            assert_eq!(console.carry_flag, true);
            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, false);

            assert_eq!(cycles, 3);
        }

        {
            setup_instruction(&mut console, vec![0x_75, 0x_86]);
            console.x_register = 0x_39;
            *console.memory_mut(0x_BF) = 0x_D1;

            console.accumulator = 0x_43;
            console.carry_flag = true;
            console.zero_flag = true;
            console.negative_flag = false;

            let cycles = execute_instruction(&mut console, adc_instruction);

            assert_eq!(console.accumulator, 0x_15);
            assert_eq!(console.carry_flag, true);
            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, false);

            assert_eq!(cycles, 4);
        }

        {
            setup_instruction(&mut console, vec![0x_6D, 0x_A6, 0x_03]);
            *console.memory_mut(0x_03A6) = 0x_DB;

            console.accumulator = 0x_37;
            console.carry_flag = true;
            console.zero_flag = true;
            console.negative_flag = true;

            let cycles = execute_instruction(&mut console, adc_instruction);

            assert_eq!(console.accumulator, 0x_13);
            assert_eq!(console.carry_flag, true);
            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, false);

            assert_eq!(cycles, 4);
        }

        {
            setup_instruction(&mut console, vec![0x_7D, 0x_DB, 0x_04]);
            console.x_register = 0x_A6;
            *console.memory_mut(0x_0581) = 0x_41;

            console.accumulator = 0x_50;
            console.carry_flag = true;
            console.zero_flag = true;
            console.negative_flag = false;

            let cycles = execute_instruction(&mut console, adc_instruction);

            assert_eq!(console.accumulator, 0x_92);
            assert_eq!(console.carry_flag, false);
            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, true);

            assert_eq!(cycles, 4 + 1);
        }

        {
            setup_instruction(&mut console, vec![0x_79, 0x_DB, 0x_04]);
            console.y_register = 0x_A6;
            *console.memory_mut(0x_0581) = 0x_41;

            console.accumulator = 0x_50;
            console.carry_flag = true;
            console.zero_flag = true;
            console.negative_flag = false;

            let cycles = execute_instruction(&mut console, adc_instruction);

            assert_eq!(console.accumulator, 0x_92);
            assert_eq!(console.carry_flag, false);
            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, true);

            assert_eq!(cycles, 5);
        }

        {
            setup_instruction(&mut console, vec![0x_61, 0x_60]);
            console.x_register = 0x_B9;
            *console.memory_mut(0x_19) = 0x_79;
            *console.memory_mut(0x_1A) = 0x_02;
            *console.memory_mut(0x_0279) = 0x_E5;

            console.accumulator = 0x_50;
            console.carry_flag = true;
            console.zero_flag = true;
            console.negative_flag = true;

            let cycles = execute_instruction(&mut console, adc_instruction);

            assert_eq!(console.accumulator, 0x_36);
            assert_eq!(console.carry_flag, true);
            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, false);

            assert_eq!(cycles, 6);
        }
    }

    #[test]
    fn test_adc_instruction_indirect_indexed() {

        let mut console = Console::new(Cartridge::new(vec![]));

        {
            setup_instruction(&mut console, vec![0x_71, 0x_42]);
            console.y_register = 0x_B7;
            *console.memory_mut(0x_42)     = 0x_24;
            *console.memory_mut(0x_42 + 1) = 0x_11;

            console.carry_flag = false;
            console.accumulator = 0x_00;
            *console.memory_mut(0x_11DB) = 0x_FF;

            let cycles = execute_instruction(&mut console, adc_instruction);

            assert_eq!(console.accumulator, 0x_FF);

            assert_eq!(cycles, 5);
        }

        {
            setup_instruction(&mut console, vec![0x_71, 0x_42]);
            console.y_register = 0x_87;
            *console.memory_mut(0x_42)     = 0x_A3;
            *console.memory_mut(0x_42 + 1) = 0x_11;

            console.carry_flag = false;
            console.accumulator = 0x_00;
            *console.memory_mut(0x_122A) = 0x_FF;

            let cycles = execute_instruction(&mut console, adc_instruction);

            assert_eq!(console.accumulator, 0x_FF);

            assert_eq!(cycles, 6);
        }
    }

    #[test]
    fn test_and_instruction() {

        // TODO; To be implemented, but frankly, the instruction and if the
        // other unit tests are passing, that instruction is high likely to be
        // correct. See ADC instruction.
        let mut console = Console::new(Cartridge::new(vec![]));

        {
            setup_instruction(&mut console, vec![0x_29, 0x_42]);

            console.accumulator = 0x_F0;
            console.zero_flag = true;
            console.negative_flag = true;

            let cycles = execute_instruction(&mut console, and_instruction);

            assert_eq!(console.accumulator, 0x_40);
            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, false);

            assert_eq!(cycles, 2);
        }
    }

    #[test]
    fn test_asl_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));

        {
            setup_instruction(&mut console, vec![0x_0A]);

            console.accumulator = 0x_42;
            console.carry_flag = true;
            console.zero_flag = true;
            console.negative_flag = false;

            let cycles = execute_instruction(&mut console, asl_instruction);

            assert_eq!(console.accumulator, 0x_84);
            assert_eq!(console.carry_flag, false);
            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, true);

            assert_eq!(cycles, 2);
        }

        {
            setup_instruction(&mut console, vec![0x_06, 127]);

            *console.memory_mut(127) = 0x_42;
            console.carry_flag = true;
            console.zero_flag = true;
            console.negative_flag = false;

            let cycles = execute_instruction(&mut console, asl_instruction);

            assert_eq!(*console.memory(127), 0x_84);
            assert_eq!(console.carry_flag, false);
            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, true);

            assert_eq!(cycles, 5);
        }
    }

    #[test]
    fn test_bcc_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));

        // Check if it's not branching on C == 1.
        setup_instruction_x(&mut console, vec![0x_90, 0x_42], 0);
        console.carry_flag = true;
        let cycles = execute_instruction(&mut console, bcc_instruction);

        assert_eq!(console.pointer_counter, 2);
        assert_eq!(cycles, 2);

        // Check branching with positive operand, without crossing page.
        setup_instruction_x(&mut console, vec![0x_90, 0x_42], 0);
        console.carry_flag = false;
        let cycles = execute_instruction(&mut console, bcc_instruction);

        assert_eq!(console.pointer_counter, 2 + 0x_42);
        assert_eq!(cycles, 3);

        // Check branching with negative operand, without crossing page.
        setup_instruction_x(&mut console, vec![0x90, 0x_F0], 0x_42);

        console.carry_flag = false;
        let cycles = execute_instruction(&mut console, bcc_instruction);

        assert_eq!(console.pointer_counter, 2 + 0x_32);
        assert_eq!(cycles, 3);

        // Check branching with positive operand, with crossing page.
        setup_instruction_x(&mut console, vec![0x90, 0x_6F], 0x_AE);

        console.carry_flag = false;
        let cycles = execute_instruction(&mut console, bcc_instruction);

        assert_eq!(console.pointer_counter, 0x_11F);
        assert_eq!(cycles, 4);

        // Check branching with negative operand, with crossing page.
        setup_instruction_x(&mut console, vec![0x90, 0x_80], 0x_05);

        console.carry_flag = false;
        let cycles = execute_instruction(&mut console, bcc_instruction);

        assert_eq!(console.pointer_counter, 0x_FF87);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_bcs_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));

        // Check if it's not branching on C == 0.
        setup_instruction_x(&mut console, vec![0xB0, 0x_42], 0);
        console.carry_flag = false;
        let cycles = execute_instruction(&mut console, bcs_instruction);

        assert_eq!(console.pointer_counter, 2);
        assert_eq!(cycles, 2);

        // Check branching with positive operand, without crossing page.
        setup_instruction_x(&mut console, vec![0xB0, 0x_42], 0);
        console.carry_flag = true;
        let cycles = execute_instruction(&mut console, bcs_instruction);

        assert_eq!(console.pointer_counter, 2 + 0x_42);
        assert_eq!(cycles, 3);

        // Check branching with negative operand, without crossing page.
        setup_instruction_x(&mut console, vec![0xB0, 0x_F0], 0x_42);

        console.carry_flag = true;
        let cycles = execute_instruction(&mut console, bcs_instruction);

        assert_eq!(console.pointer_counter, 2 + 0x_32);
        assert_eq!(cycles, 3);

        // Check branching with positive operand, with crossing page.
        setup_instruction_x(&mut console, vec![0xB0, 0x_6F], 0x_AE);

        console.carry_flag = true;
        let cycles = execute_instruction(&mut console, bcs_instruction);

        assert_eq!(console.pointer_counter, 0x_11F);
        assert_eq!(cycles, 4);

        // Check branching with negative operand, with crossing page.
        setup_instruction_x(&mut console, vec![0xB0, 0x_80], 0x_05);

        console.carry_flag = true;
        let cycles = execute_instruction(&mut console, bcs_instruction);

        assert_eq!(console.pointer_counter, 0x_FF87);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_beq_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));

        // Check if it's not branching on Z == 0.
        setup_instruction_x(&mut console, vec![0x_F0, 0x_42], 0);
        console.zero_flag = false;
        let cycles = execute_instruction(&mut console, beq_instruction);

        assert_eq!(console.pointer_counter, 2);
        assert_eq!(cycles, 2);

        // Check branching with positive operand, without crossing page.
        setup_instruction_x(&mut console, vec![0x_F0, 0x_42], 0);
        console.zero_flag = true;
        let cycles = execute_instruction(&mut console, beq_instruction);

        assert_eq!(console.pointer_counter, 2 + 0x_42);
        assert_eq!(cycles, 3);

        // Check branching with negative operand, without crossing page.
        setup_instruction_x(&mut console, vec![0x_F0, 0x_F0], 0x_42);

        console.zero_flag = true;
        let cycles = execute_instruction(&mut console, beq_instruction);

        assert_eq!(console.pointer_counter, 2 + 0x_32);
        assert_eq!(cycles, 3);

        // Check branching with positive operand, with crossing page.
        setup_instruction_x(&mut console, vec![0x_F0, 0x_6F], 0x_AE);

        console.zero_flag = true;
        let cycles = execute_instruction(&mut console, beq_instruction);

        assert_eq!(console.pointer_counter, 0x_11F);
        assert_eq!(cycles, 4);

        // Check branching with negative operand, with crossing page.
        setup_instruction_x(&mut console, vec![0x_F0, 0x_80], 0x_05);

        console.zero_flag = true;
        let cycles = execute_instruction(&mut console, beq_instruction);

        assert_eq!(console.pointer_counter, 0x_FF87);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_bit_instruction() {
        let mut console = Console::new(Cartridge::new(vec![]));

        {
            setup_instruction(&mut console, vec![0x_24, 0x_42]);

            *console.memory_mut(0x_42) = 0x_40;
            console.negative_flag = true;
            console.overflow_flag = false;

            console.accumulator = 0x_00;
            console.zero_flag = false;

            let cycles = execute_instruction(&mut console, bit_instruction);

            assert_eq!(console.negative_flag, false);
            assert_eq!(console.overflow_flag, true);

            assert_eq!(console.zero_flag, true);

            assert_eq!(cycles, 3);
        }

        {
            setup_instruction(&mut console, vec![0x_24, 0x_42]);

            *console.memory_mut(0x_42) = 0x_80;
            console.negative_flag = false;
            console.overflow_flag = true;

            console.accumulator = 0x_80;
            console.zero_flag = true;

            let cycles = execute_instruction(&mut console, bit_instruction);

            assert_eq!(console.negative_flag, true);
            assert_eq!(console.overflow_flag, false);

            assert_eq!(console.zero_flag, false);

            assert_eq!(cycles, 3);
        }

    }

    #[test]
    fn test_bmi_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));

        // Check if it's not branching on N == 0.
        setup_instruction_x(&mut console, vec![0x30, 0x_42], 0);
        console.negative_flag = false;
        let cycles = execute_instruction(&mut console, bmi_instruction);

        assert_eq!(console.pointer_counter, 2);
        assert_eq!(cycles, 2);

        // Check branching with positive operand, without crossing page.
        setup_instruction_x(&mut console, vec![0x30, 0x_42], 0);
        console.negative_flag = true;
        let cycles = execute_instruction(&mut console, bmi_instruction);

        assert_eq!(console.pointer_counter, 2 + 0x_42);
        assert_eq!(cycles, 3);

        // Check branching with negative operand, without crossing page.
        setup_instruction_x(&mut console, vec![0x30, 0x_F0], 0x_42);

        console.negative_flag = true;
        let cycles = execute_instruction(&mut console, bmi_instruction);

        assert_eq!(console.pointer_counter, 2 + 0x_32);
        assert_eq!(cycles, 3);

        // Check branching with positive operand, with crossing page.
        setup_instruction_x(&mut console, vec![0x30, 0x_6F], 0x_AE);

        console.negative_flag = true;
        let cycles = execute_instruction(&mut console, bmi_instruction);

        assert_eq!(console.pointer_counter, 0x_11F);
        assert_eq!(cycles, 4);

        // Check branching with negative operand, with crossing page.
        setup_instruction_x(&mut console, vec![0x30, 0x_80], 0x_05);

        console.negative_flag = true;
        let cycles = execute_instruction(&mut console, bmi_instruction);

        assert_eq!(console.pointer_counter, 0x_FF87);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_bne_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));

        // Check if it's not branching on Z == 1.
        setup_instruction_x(&mut console, vec![0x_D0, 0x_42], 0);
        console.zero_flag = true;
        let cycles = execute_instruction(&mut console, bne_instruction);

        assert_eq!(console.pointer_counter, 2);
        assert_eq!(cycles, 2);

        // Check branching with positive operand, without crossing page.
        setup_instruction_x(&mut console, vec![0x_D0, 0x_42], 0);
        console.zero_flag = false;
        let cycles = execute_instruction(&mut console, bne_instruction);

        assert_eq!(console.pointer_counter, 2 + 0x_42);
        assert_eq!(cycles, 3);

        // Check branching with negative operand, without crossing page.
        setup_instruction_x(&mut console, vec![0x_D0, 0x_F0], 0x_42);

        console.zero_flag = false;
        let cycles = execute_instruction(&mut console, bne_instruction);

        assert_eq!(console.pointer_counter, 2 + 0x_32);
        assert_eq!(cycles, 3);

        // Check branching with positive operand, with crossing page.
        setup_instruction_x(&mut console, vec![0x_D0, 0x_6F], 0x_AE);

        console.zero_flag = false;
        let cycles = execute_instruction(&mut console, bne_instruction);

        assert_eq!(console.pointer_counter, 0x_11F);
        assert_eq!(cycles, 4);

        // Check branching with negative operand, with crossing page.
        setup_instruction_x(&mut console, vec![0x_D0, 0x_80], 0x_05);

        console.zero_flag = false;
        let cycles = execute_instruction(&mut console, bne_instruction);

        assert_eq!(console.pointer_counter, 0x_FF87);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_bpl_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));

        // Check if it's not branching on N == 1.
        setup_instruction_x(&mut console, vec![0x10, 0x_42], 0);
        console.negative_flag = true;
        let cycles = execute_instruction(&mut console, bpl_instruction);

        assert_eq!(console.pointer_counter, 2);
        assert_eq!(cycles, 2);

        // Check branching with positive operand, without crossing page.
        setup_instruction_x(&mut console, vec![0x10, 0x_42], 0);
        console.negative_flag = false;
        let cycles = execute_instruction(&mut console, bpl_instruction);

        assert_eq!(console.pointer_counter, 2 + 0x_42);
        assert_eq!(cycles, 3);

        // Check branching with negative operand, without crossing page.
        setup_instruction_x(&mut console, vec![0x10, 0x_F0], 0x_42);

        console.negative_flag = false;
        let cycles = execute_instruction(&mut console, bpl_instruction);

        assert_eq!(console.pointer_counter, 2 + 0x_32);
        assert_eq!(cycles, 3);

        // Check branching with positive operand, with crossing page.
        setup_instruction_x(&mut console, vec![0x10, 0x_6F], 0x_AE);

        console.negative_flag = false;
        let cycles = execute_instruction(&mut console, bpl_instruction);

        assert_eq!(console.pointer_counter, 0x_11F);
        assert_eq!(cycles, 4);

        // Check branching with negative operand, with crossing page.
        setup_instruction_x(&mut console, vec![0x10, 0x_80], 0x_05);

        console.negative_flag = false;
        let cycles = execute_instruction(&mut console, bpl_instruction);

        assert_eq!(console.pointer_counter, 0x_FF87);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_brk_instruction() {
        // To be implemented.
    }

    #[test]
    fn test_bvc_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));

        // Check if it's not branching on V == 1.
        setup_instruction_x(&mut console, vec![0x_50, 0x_42], 0);
        console.overflow_flag = true;
        let cycles = execute_instruction(&mut console, bvc_instruction);

        assert_eq!(console.pointer_counter, 2);
        assert_eq!(cycles, 2);

        // Check branching with positive operand, without crossing page.
        setup_instruction_x(&mut console, vec![0x_50, 0x_42], 0);
        console.overflow_flag = false;
        let cycles = execute_instruction(&mut console, bvc_instruction);

        assert_eq!(console.pointer_counter, 2 + 0x_42);
        assert_eq!(cycles, 3);

        // Check branching with negative operand, without crossing page.
        setup_instruction_x(&mut console, vec![0x_50, 0x_F0], 0x_42);

        console.overflow_flag = false;
        let cycles = execute_instruction(&mut console, bvc_instruction);

        assert_eq!(console.pointer_counter, 2 + 0x_32);
        assert_eq!(cycles, 3);

        // Check branching with positive operand, with crossing page.
        setup_instruction_x(&mut console, vec![0x_50, 0x_6F], 0x_AE);

        console.overflow_flag = false;
        let cycles = execute_instruction(&mut console, bvc_instruction);

        assert_eq!(console.pointer_counter, 0x_11F);
        assert_eq!(cycles, 4);

        // Check branching with negative operand, with crossing page.
        setup_instruction_x(&mut console, vec![0x_50, 0x_80], 0x_05);

        console.overflow_flag = false;
        let cycles = execute_instruction(&mut console, bvc_instruction);

        assert_eq!(console.pointer_counter, 0x_FF87);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_bvs_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));

        // Check if it's not branching on V == 0.
        setup_instruction_x(&mut console, vec![0x_70, 0x_42], 0);
        console.overflow_flag = false;
        let cycles = execute_instruction(&mut console, bvs_instruction);

        assert_eq!(console.pointer_counter, 2);
        assert_eq!(cycles, 2);

        // Check branching with positive operand, without crossing page.
        setup_instruction_x(&mut console, vec![0x_70, 0x_42], 0);
        console.overflow_flag = true;
        let cycles = execute_instruction(&mut console, bvs_instruction);

        assert_eq!(console.pointer_counter, 2 + 0x_42);
        assert_eq!(cycles, 3);

        // Check branching with negative operand, without crossing page.
        setup_instruction_x(&mut console, vec![0x_70, 0x_F0], 0x_42);

        console.overflow_flag = true;
        let cycles = execute_instruction(&mut console, bvs_instruction);

        assert_eq!(console.pointer_counter, 2 + 0x_32);
        assert_eq!(cycles, 3);

        // Check branching with positive operand, with crossing page.
        setup_instruction_x(&mut console, vec![0x_70, 0x_6F], 0x_AE);

        console.overflow_flag = true;
        let cycles = execute_instruction(&mut console, bvs_instruction);

        assert_eq!(console.pointer_counter, 0x_11F);
        assert_eq!(cycles, 4);

        // Check branching with negative operand, with crossing page.
        setup_instruction_x(&mut console, vec![0x_70, 0x_80], 0x_05);

        console.overflow_flag = true;
        let cycles = execute_instruction(&mut console, bvs_instruction);

        assert_eq!(console.pointer_counter, 0x_FF87);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_clc_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));
        setup_instruction(&mut console, vec![0x_18]);

        console.carry_flag = true;
        let cycles = execute_instruction(&mut console, clc_instruction);
        assert_eq!(console.carry_flag, false);

        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_cld_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));
        setup_instruction(&mut console, vec![0x_D8]);

        console.decimal_flag = true;
        let cycles = execute_instruction(&mut console, cld_instruction);
        assert_eq!(console.decimal_flag, false);

        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_cli_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));
        setup_instruction(&mut console, vec![0x_58]);

        console.interrupt_flag = true;
        let cycles = execute_instruction(&mut console, cli_instruction);
        assert_eq!(console.interrupt_flag, false);

        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_clv_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));
        setup_instruction(&mut console, vec![0x_B8]);

        console.overflow_flag = true;
        let cycles = execute_instruction(&mut console, clv_instruction);
        assert_eq!(console.overflow_flag, false);

        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_cmp_instruction() {

        // It doesn't test the different addressing mode because it's already
        // tested by the other instructions. Perhaps the number of cycles should
        // be tested though.
        let mut console = Console::new(Cartridge::new(vec![]));

        {
            setup_instruction(&mut console, vec![0x_C9, 0x_41]);
            console.accumulator = 0x_42;

            console.carry_flag = false;
            console.zero_flag = true;
            console.negative_flag = true;

            let cycles = execute_instruction(&mut console, cmp_instruction);

            assert_eq!(console.carry_flag, true);
            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, false);

            assert_eq!(cycles, 2);
        }

        {
            setup_instruction(&mut console, vec![0x_C9, 0x_42]);
            console.accumulator = 0x_42;

            console.carry_flag = false;
            console.zero_flag = false;
            console.negative_flag = true;

            let cycles = execute_instruction(&mut console, cmp_instruction);

            assert_eq!(console.carry_flag, true);
            assert_eq!(console.zero_flag, true);
            assert_eq!(console.negative_flag, false);

            assert_eq!(cycles, 2);
        }

        {
            setup_instruction(&mut console, vec![0x_C9, 0x_43]);
            console.accumulator = 0x_42;

            console.carry_flag = true;
            console.zero_flag = true;
            console.negative_flag = false;

            let cycles = execute_instruction(&mut console, cmp_instruction);

            assert_eq!(console.carry_flag, false);
            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, true);

            assert_eq!(cycles, 2);
        }
    }

    #[test]
    fn test_cpx_instruction() {

        // It doesn't test the different adressing mode because it's already
        // tested by the other instructions.
        let mut console = Console::new(Cartridge::new(vec![]));

        {
            setup_instruction(&mut console, vec![0x_E0, 0x_41]);
            console.x_register = 0x_42;

            console.carry_flag = false;
            console.zero_flag = true;
            console.negative_flag = true;

            let cycles = execute_instruction(&mut console, cpx_instruction);

            assert_eq!(console.carry_flag, true);
            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, false);

            assert_eq!(cycles, 2);
        }

        {
            setup_instruction(&mut console, vec![0x_E0, 0x_42]);
            console.x_register = 0x_42;

            console.carry_flag = false;
            console.zero_flag = false;
            console.negative_flag = true;

            let cycles = execute_instruction(&mut console, cpx_instruction);

            assert_eq!(console.carry_flag, true);
            assert_eq!(console.zero_flag, true);
            assert_eq!(console.negative_flag, false);

            assert_eq!(cycles, 2);
        }

        {
            setup_instruction(&mut console, vec![0x_E0, 0x_43]);
            console.x_register = 0x_42;

            console.carry_flag = true;
            console.zero_flag = true;
            console.negative_flag = false;

            let cycles = execute_instruction(&mut console, cpx_instruction);

            assert_eq!(console.carry_flag, false);
            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, true);

            assert_eq!(cycles, 2);
        }
    }

    #[test]
    fn test_cpy_instruction() {

        // It doesn't test the different adressing mode because it's already
        // tested by the other instructions.
        let mut console = Console::new(Cartridge::new(vec![]));

        {
            setup_instruction(&mut console, vec![0x_C0, 0x_41]);
            console.y_register = 0x_42;

            console.carry_flag = false;
            console.zero_flag = true;
            console.negative_flag = true;

            let cycles = execute_instruction(&mut console, cpy_instruction);

            assert_eq!(console.carry_flag, true);
            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, false);

            assert_eq!(cycles, 2);
        }

        {
            setup_instruction(&mut console, vec![0x_C0, 0x_42]);
            console.y_register = 0x_42;

            console.carry_flag = false;
            console.zero_flag = false;
            console.negative_flag = true;

            let cycles = execute_instruction(&mut console, cpy_instruction);

            assert_eq!(console.carry_flag, true);
            assert_eq!(console.zero_flag, true);
            assert_eq!(console.negative_flag, false);

            assert_eq!(cycles, 2);
        }

        {
            setup_instruction(&mut console, vec![0x_C0, 0x_43]);
            console.y_register = 0x_42;

            console.carry_flag = true;
            console.zero_flag = true;
            console.negative_flag = false;

            let cycles = execute_instruction(&mut console, cpy_instruction);

            assert_eq!(console.carry_flag, false);
            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, true);

            assert_eq!(cycles, 2);
        }
    }

    #[test]
    fn test_dec_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));

        {
            setup_instruction(&mut console, vec![0x_C6, 0x_42]);

            *console.memory_mut(0x_42) = 128;
            console.negative_flag = true;
            console.zero_flag = true;

            let cycles = execute_instruction(&mut console, dec_instruction);

            assert_eq!(*console.memory(0x_42), 127);
            assert_eq!(console.negative_flag, false);
            assert_eq!(console.zero_flag, false);

            assert_eq!(cycles, 5);
        }

        {
            setup_instruction(&mut console, vec![0x_D6, 0x_41]);
            console.x_register = 0x_01;

            *console.memory_mut(0x_42) = 128;
            console.negative_flag = true;
            console.zero_flag = true;

            let cycles = execute_instruction(&mut console, dec_instruction);

            assert_eq!(*console.memory(0x_42), 127);
            assert_eq!(console.negative_flag, false);
            assert_eq!(console.zero_flag, false);

            assert_eq!(cycles, 6);
        }

        {
            setup_instruction(&mut console, vec![0x_CE, 0x_42, 3]);

            *console.memory_mut(3 * 256 + 0x_42) = 128;
            console.negative_flag = true;
            console.zero_flag = true;

            let cycles = execute_instruction(&mut console, dec_instruction);

            assert_eq!(*console.memory(3 * 256 + 0x_42), 127);
            assert_eq!(console.negative_flag, false);
            assert_eq!(console.zero_flag, false);

            assert_eq!(cycles, 6);
        }

        {
            setup_instruction(&mut console, vec![0x_DE, 0x_41, 3]);
            console.x_register = 0x_01;

            *console.memory_mut(3 * 256 + 0x_42) = 128;
            console.negative_flag = true;
            console.zero_flag = true;

            let cycles = execute_instruction(&mut console, dec_instruction);

            assert_eq!(*console.memory(3 * 256 + 0x_42), 127);
            assert_eq!(console.negative_flag, false);
            assert_eq!(console.zero_flag, false);

            assert_eq!(cycles, 7);
        }
    }

    #[test]
    fn test_dex_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));

        setup_instruction(&mut console, vec![0x_CA]);

        console.x_register = 128;
        console.negative_flag = true;
        console.zero_flag = true;

        let cycles = execute_instruction(&mut console, dex_instruction);

        assert_eq!(console.x_register, 127);
        assert_eq!(console.negative_flag, false);
        assert_eq!(console.zero_flag, false);

        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_dey_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));
        setup_instruction(&mut console, vec![0x_88]);

        console.y_register = 128;
        console.negative_flag = true;
        console.zero_flag = true;

        let cycles = execute_instruction(&mut console, dey_instruction);

        assert_eq!(console.y_register, 127);
        assert_eq!(console.negative_flag, false);
        assert_eq!(console.zero_flag, false);

        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_eor_instruction() {

        // It doesn't test the different adressing mode because it's already
        // tested by the other instructions. Perhaps the number of cycles should
        // be tested though.
        let mut console = Console::new(Cartridge::new(vec![]));
        setup_instruction(&mut console, vec![0x_49, 0x_55]);

        console.accumulator = 0x_33;
        console.zero_flag = true;
        console.negative_flag = true;

        let cycles = execute_instruction(&mut console, eor_instruction);

        assert_eq!(console.accumulator, 0x_66);
        assert_eq!(console.zero_flag, false);
        assert_eq!(console.negative_flag, false);

        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_inc_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));

        {
            setup_instruction(&mut console, vec![0x_E6, 0x_42]);

            *console.memory_mut(0x_42) = 127;
            console.negative_flag = false;
            console.zero_flag = true;

            let cycles = execute_instruction(&mut console, inc_instruction);

            assert_eq!(*console.memory(0x_42), 128);
            assert_eq!(console.negative_flag, true);
            assert_eq!(console.zero_flag, false);

            assert_eq!(cycles, 5);
        }

        {
            setup_instruction(&mut console, vec![0x_F6, 0x_41]);
            console.x_register = 0x_01;

            *console.memory_mut(0x_42) = 127;
            console.negative_flag = false;
            console.zero_flag = true;

            let cycles = execute_instruction(&mut console, inc_instruction);

            assert_eq!(*console.memory(0x_42), 128);
            assert_eq!(console.negative_flag, true);
            assert_eq!(console.zero_flag, false);

            assert_eq!(cycles, 6);
        }

        {
            setup_instruction(&mut console, vec![0x_EE, 0x_42, 3]);

            *console.memory_mut(3 * 256 + 0x_42) = 127;
            console.negative_flag = false;
            console.zero_flag = true;

            let cycles = execute_instruction(&mut console, inc_instruction);

            assert_eq!(*console.memory(3 * 256 + 0x_42), 128);
            assert_eq!(console.negative_flag, true);
            assert_eq!(console.zero_flag, false);

            assert_eq!(cycles, 6);
        }

        {
            setup_instruction(&mut console, vec![0x_FE, 0x_41, 3]);
            console.x_register = 0x_01;

            *console.memory_mut(3 * 256 + 0x_42) = 127;
            console.negative_flag = false;
            console.zero_flag = true;

            let cycles = execute_instruction(&mut console, inc_instruction);

            assert_eq!(*console.memory(3 * 256 + 0x_42), 128);
            assert_eq!(console.negative_flag, true);
            assert_eq!(console.zero_flag, false);

            assert_eq!(cycles, 7);
        }
    }

    #[test]
    fn test_inx_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));

        setup_instruction(&mut console, vec![0x_E8]);

        console.x_register = 127;
        console.negative_flag = false;
        console.zero_flag = true;

        let cycles = execute_instruction(&mut console, inx_instruction);

        assert_eq!(console.x_register, 128);
        assert_eq!(console.negative_flag, true);
        assert_eq!(console.zero_flag, false);

        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_iny_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));
        setup_instruction(&mut console, vec![0x_C8]);

        console.y_register = 127;
        console.negative_flag = false;
        console.zero_flag = true;

        let cycles = execute_instruction(&mut console, iny_instruction);

        assert_eq!(console.y_register, 128);
        assert_eq!(console.negative_flag, true);
        assert_eq!(console.zero_flag, false);

        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_jmp_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));

        {
            setup_instruction(&mut console, vec![0x_4C, 0x_42, 0x_31]);
            let cycles = execute_instruction(&mut console, jmp_instruction);

            assert_eq!(console.pointer_counter, 0x_3142);
            assert_eq!(cycles, 3);
        }

        {
            setup_instruction(&mut console, vec![0x_6C, 0x_11, 0x_22]);
            *console.memory_mut(0x_2211)     = 0x_42;
            *console.memory_mut(0x_2211 + 1) = 0x_31;

            let cycles = execute_instruction(&mut console, jmp_instruction);

            assert_eq!(console.pointer_counter, 0x_3142);
            assert_eq!(cycles, 5);
        }
    }

    #[test]
    fn test_jsr_instruction() {
        let mut console = Console::new(Cartridge::new(vec![]));

        setup_instruction(&mut console, vec![0x_20, 0x_42, 0x_31]);
        let pointer_counter = console.pointer_counter;

        let cycles = execute_instruction(&mut console, jsr_instruction);

        let ll = console.pop_value();
        let hh = console.pop_value();
        assert_eq!(u16::from_le_bytes([ll, hh]), pointer_counter + 2);

        assert_eq!(console.pointer_counter, 0x_3142);

        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_lda_instruction() {

        // It doesn't test the different adressing mode because it's already
        // tested by the other instructions. Perhaps the number of cycles should
        // be tested though.
        let mut console = Console::new(Cartridge::new(vec![]));

        {
            setup_instruction(&mut console, vec![0x_A9, 128]);

            console.accumulator = 127;
            console.zero_flag = true;
            console.negative_flag = false;

            let cycles = execute_instruction(&mut console, lda_instruction);

            assert_eq!(console.accumulator, 128);
            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, true);

            assert_eq!(cycles, 2);
        }
    }

    #[test]
    fn test_ldx_instruction() {

        // It doesn't test the different adressing mode because it's already
        // tested by the other instructions. Perhaps the number of cycles should
        // be tested though.
        let mut console = Console::new(Cartridge::new(vec![]));

        {
            setup_instruction(&mut console, vec![0x_A2, 128]);

            console.x_register = 127;
            console.zero_flag = true;
            console.negative_flag = false;

            let cycles = execute_instruction(&mut console, ldx_instruction);

            assert_eq!(console.x_register, 128);
            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, true);

            assert_eq!(cycles, 2);
        }
    }

    #[test]
    fn test_ldy_instruction() {

        // It doesn't test the different adressing mode because it's already
        // tested by the other instructions. Perhaps the number of cycles should
        // be tested though.
        let mut console = Console::new(Cartridge::new(vec![]));

        {
            setup_instruction(&mut console, vec![0x_A0, 128]);

            console.y_register = 127;
            console.zero_flag = true;
            console.negative_flag = false;

            let cycles = execute_instruction(&mut console, ldy_instruction);

            assert_eq!(console.y_register, 128);
            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, true);

            assert_eq!(cycles, 2);
        }
    }

    #[test]
    fn test_lsr_instruction() {

        // It doesn't test the different adressing mode because it's already
        // tested by the other instructions. Perhaps the number of cycles should
        // be tested though.
        let mut console = Console::new(Cartridge::new(vec![]));

        {
            setup_instruction(&mut console, vec![0x_4A]);

            console.carry_flag = true;
            console.accumulator = 0x_AA;

            console.zero_flag = true;
            console.negative_flag = true;

            let cycles = execute_instruction(&mut console, lsr_instruction);

            console.carry_flag = true;
            assert_eq!(console.accumulator, 0x_55);

            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, false);

            assert_eq!(cycles, 2);
        }

        {
            setup_instruction(&mut console, vec![0x_46, 0x_42]);

            console.carry_flag = true;
            *console.memory_mut(0x_42) = 0x_AA;

            console.zero_flag = true;
            console.negative_flag = true;

            let cycles = execute_instruction(&mut console, lsr_instruction);

            console.carry_flag = true;
            assert_eq!(*console.memory(0x_42), 0x_55);

            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, false);

            assert_eq!(cycles, 5);
        }
    }

    #[test]
    fn test_nop_instruction() {
        let mut console = Console::new(Cartridge::new(vec![]));
        setup_instruction(&mut console, vec![0x_EA]);

        let pointer_counter = console.pointer_counter;

        console.accumulator = 0x_4B;
        console.x_register = 0x_E1;
        console.y_register = 0x_CD;

        console.negative_flag = true;
        console.overflow_flag = false;
        console.break_flag = true;
        console.decimal_flag = false;
        console.interrupt_flag = true;
        console.zero_flag = false;
        console.carry_flag = true;

        let cycles = execute_instruction(&mut console, nop_instruction);

        assert_eq!(console.pointer_counter, pointer_counter + 1);

        assert_eq!(console.accumulator, 0x_4B);
        assert_eq!(console.x_register, 0x_E1);
        assert_eq!(console.y_register, 0x_CD);

        assert_eq!(console.negative_flag, true);
        assert_eq!(console.overflow_flag, false);
        assert_eq!(console.break_flag, true);
        assert_eq!(console.decimal_flag, false);
        assert_eq!(console.interrupt_flag, true);
        assert_eq!(console.zero_flag, false);
        assert_eq!(console.carry_flag, true);

        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_ora_instruction() {

        // It doesn't test the different adressing mode because it's already
        // tested by the other instructions. Perhaps the number of cycles should
        // be tested though.
        let mut console = Console::new(Cartridge::new(vec![]));

        {
            setup_instruction(&mut console, vec![0x_09, 0x_55]);

            console.accumulator = 0x_33;
            console.zero_flag = true;
            console.negative_flag = true;

            let cycles = execute_instruction(&mut console, ora_instruction);

            assert_eq!(console.accumulator, 0x_77);
            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, false);

            assert_eq!(cycles, 2);
        }
    }

    #[test]
    fn test_pha_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));
        setup_instruction(&mut console, vec![0x_48]);

        console.accumulator = 0x_42;
        *console.memory_mut(0x_FF) = 0x_00;

        let cycles = execute_instruction(&mut console, pha_instruction);

        assert_eq!(console.accumulator, 0x_42);
        assert_eq!(*console.memory(0x_FF), 0x_42);

        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_php_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));
        setup_instruction(&mut console, vec![0x_08]);

        console.negative_flag  = true;
        console.overflow_flag  = false;
        console.break_flag     = false;
        console.decimal_flag   = true;
        console.interrupt_flag = false;
        console.zero_flag      = true;
        console.carry_flag     = false;
        *console.memory_mut(0x_FF) = 0x_00;

        let cycles = execute_instruction(&mut console, php_instruction);

        assert_eq!(*console.memory(0x_FF), 0b1000_1010);

        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_pla_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));
        setup_instruction(&mut console, vec![0x_68]);

        console.push_value(0x_42);
        console.accumulator = 0x_00;

        let cycles = execute_instruction(&mut console, pla_instruction);
        assert_eq!(console.accumulator, 0x_42);

        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_plp_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));
        setup_instruction(&mut console, vec![0x_28]);

        console.push_value(0b1000_1010);
        console.negative_flag  = false;
        console.overflow_flag  = true;
        console.break_flag     = true;
        console.decimal_flag   = false;
        console.interrupt_flag = true;
        console.zero_flag      = false;
        console.carry_flag     = true;

        let cycles = execute_instruction(&mut console, plp_instruction);
        assert_eq!(console.negative_flag, true);
        assert_eq!(console.overflow_flag, false);
        assert_eq!(console.break_flag, false);
        assert_eq!(console.decimal_flag, true);
        assert_eq!(console.interrupt_flag, false);
        assert_eq!(console.zero_flag, true);
        assert_eq!(console.carry_flag, false);

        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_rol_instruction() {

        // It doesn't test the different adressing mode because it's already
        // tested by the other instructions. Perhaps the number of cycles should
        // be tested though.
        let mut console = Console::new(Cartridge::new(vec![]));

        {
            setup_instruction(&mut console, vec![0x_2A]);

            console.carry_flag = false;
            console.accumulator = 0x_AA;

            console.zero_flag = true;
            console.negative_flag = true;

            let cycles = execute_instruction(&mut console, rol_instruction);

            console.carry_flag = true;
            assert_eq!(console.accumulator, 0x_54);

            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, false);

            assert_eq!(cycles, 2);
        }

        {
            setup_instruction(&mut console, vec![0x_26, 0x_42]);

            console.carry_flag = false;
            *console.memory_mut(0x_42) = 0x_AA;

            console.zero_flag = true;
            console.negative_flag = true;

            let cycles = execute_instruction(&mut console, rol_instruction);

            console.carry_flag = true;
            assert_eq!(*console.memory(0x_42), 0x_54);

            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, false);

            assert_eq!(cycles, 5);
        }
    }

    #[test]
    fn test_ror_instruction() {

        // It doesn't test the different adressing mode because it's already
        // tested by the other instructions. Perhaps the number of cycles should
        // be tested though.
        let mut console = Console::new(Cartridge::new(vec![]));

        {
            setup_instruction(&mut console, vec![0x_6A]);

            console.carry_flag = true;
            console.accumulator = 0x_AA;

            console.zero_flag = true;
            console.negative_flag = false;

            let cycles = execute_instruction(&mut console, ror_instruction);

            console.carry_flag = false;
            assert_eq!(console.accumulator, 0x_D5);

            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, true);

            assert_eq!(cycles, 2);
        }

        {
            setup_instruction(&mut console, vec![0x_66, 0x_42]);

            console.carry_flag = true;
            *console.memory_mut(0x_42) = 0x_AA;

            console.zero_flag = true;
            console.negative_flag = false;

            let cycles = execute_instruction(&mut console, ror_instruction);

            console.carry_flag = false;
            assert_eq!(*console.memory(0x_42), 0x_D5);

            assert_eq!(console.zero_flag, false);
            assert_eq!(console.negative_flag, true);

            assert_eq!(cycles, 5);
        }
    }

    #[test]
    fn test_rti_instruction() {
        // To be implemetend.
    }

    #[test]
    fn test_rts_instruction() {
        // let mut console = Console::new(Cartridge::new(vec![]));

        // setup_instruction(&mut console, vec![0x_6C, 0x_42, 0x_31]);
        // *console.memory_mut(0x_3142) = 0x_60;

        // let cycles = execute_instruction(&mut console, jsr_instruction);
        // let cycles = execute_instruction(&mut console, rts_instruction);

    }

    #[test]
    fn test_sbc_instruction() {
        // To be implemetend.
    }

    #[test]
    fn test_sec_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));
        setup_instruction(&mut console, vec![0x_38]);

        console.carry_flag = false;
        let cycles = execute_instruction(&mut console, sec_instruction);
        assert_eq!(console.carry_flag, true);

        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_sed_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));
        setup_instruction(&mut console, vec![0x_F8]);

        console.decimal_flag = false;
        let cycles = execute_instruction(&mut console, sed_instruction);
        assert_eq!(console.decimal_flag, true);

        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_sei_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));
        setup_instruction(&mut console, vec![0x_78]);

        console.interrupt_flag = false;
        let cycles = execute_instruction(&mut console, sei_instruction);
        assert_eq!(console.interrupt_flag, true);

        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_sta_instruction() {

        // different address mode aren't tested here
        let mut console = Console::new(Cartridge::new(vec![]));
        setup_instruction(&mut console, vec![0x_85, 127]);

        *console.memory_mut(127) = 0;
        console.accumulator = 0x_42;

        let cycles = execute_instruction(&mut console, sta_instruction);
        assert_eq!(*console.memory(127), 0x_42);

        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_stx_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));

        {
            setup_instruction(&mut console, vec![0x_86, 127]);

            *console.memory_mut(127) = 0;
            console.x_register = 0x_42;

            let cycles = execute_instruction(&mut console, stx_instruction);
            assert_eq!(*console.memory(127), 0x_42);

            assert_eq!(cycles, 3);
        }

        {
            setup_instruction(&mut console, vec![0x_96, 127]);

            *console.memory_mut(128) = 0;
            console.x_register = 0x_42;
            console.y_register = 1;

            let cycles = execute_instruction(&mut console, stx_instruction);
            assert_eq!(*console.memory(128), 0x_42);

            assert_eq!(cycles, 4);
        }

        {
            setup_instruction(&mut console, vec![0x_8E, 0x_7F, 0x_03]);

            *console.memory_mut(0x_037F) = 0;
            console.x_register = 0x_42;

            let cycles = execute_instruction(&mut console, stx_instruction);
            assert_eq!(*console.memory(0x_037F), 0x_42);

            assert_eq!(cycles, 4);
        }
    }

    #[test]
    fn test_sty_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));

        {
            setup_instruction(&mut console, vec![0x_84, 127]);

            *console.memory_mut(127) = 0;
            console.y_register = 0x_42;

            let cycles = execute_instruction(&mut console, sty_instruction);
            assert_eq!(*console.memory(127), 0x_42);

            assert_eq!(cycles, 3);
        }

        {
            setup_instruction(&mut console, vec![0x_94, 127]);

            *console.memory_mut(128) = 0;
            console.x_register = 1;
            console.y_register = 0x_42;

            let cycles = execute_instruction(&mut console, sty_instruction);
            assert_eq!(*console.memory(128), 0x_42);

            assert_eq!(cycles, 4);
        }

        {
            setup_instruction(&mut console, vec![0x_8C, 0x_7F, 0x_03]);

            *console.memory_mut(0x_037F) = 0;
            console.y_register = 0x_42;

            let cycles = execute_instruction(&mut console, sty_instruction);
            assert_eq!(*console.memory(0x_037F), 0x_42);

            assert_eq!(cycles, 4);
        }
    }

    #[test]
    fn test_tax_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));
        setup_instruction(&mut console, vec![0x_AA]);

        console.accumulator = 42;
        console.x_register = 0;
        console.negative_flag = true;
        console.zero_flag = true;

        let cycles = execute_instruction(&mut console, tax_instruction);

        assert_eq!(console.accumulator, 42);
        assert_eq!(console.x_register, 42);
        assert_eq!(console.negative_flag, false);
        assert_eq!(console.zero_flag, false);

        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_tay_instruction() {

        let mut console = Console::new(Cartridge::new(vec![]));
        setup_instruction(&mut console, vec![0x_A8]);

        console.accumulator = 42;
        console.y_register = 0;
        console.negative_flag = true;
        console.zero_flag = true;

        let cycles = execute_instruction(&mut console, tay_instruction);

        assert_eq!(console.accumulator, 42);
        assert_eq!(console.y_register, 42);
        assert_eq!(console.negative_flag, false);
        assert_eq!(console.zero_flag, false);

        assert_eq!(cycles, 2);
    }

        #[test]
        fn test_tsx_instruction() {

            let mut console = Console::new(Cartridge::new(vec![]));
            setup_instruction(&mut console, vec![0x_BA]);

            console.x_register = 0;
            console.stack_pointer = 42;
            console.negative_flag = true;
            console.zero_flag = true;

            let cycles = execute_instruction(&mut console, tsx_instruction);

            assert_eq!(console.x_register, 42);
            assert_eq!(console.stack_pointer, 42);
            assert_eq!(console.negative_flag, false);
            assert_eq!(console.zero_flag, false);

            assert_eq!(cycles, 2);
        }

        #[test]
        fn test_txa_instruction() {

            let mut console = Console::new(Cartridge::new(vec![]));
            setup_instruction(&mut console, vec![0x_8A]);

            console.accumulator = 0;
            console.x_register = 42;
            console.negative_flag = true;
            console.zero_flag = true;

            let cycles = execute_instruction(&mut console, txa_instruction);

            assert_eq!(console.accumulator, 42);
            assert_eq!(console.x_register, 42);
            assert_eq!(console.negative_flag, false);
            assert_eq!(console.zero_flag, false);

            assert_eq!(cycles, 2);
        }

        #[test]
        fn test_txs_instruction() {

            let mut console = Console::new(Cartridge::new(vec![]));
            setup_instruction(&mut console, vec![0x_9A]);

            console.x_register = 42;
            console.stack_pointer = 0;

            let cycles = execute_instruction(&mut console, txs_instruction);

            assert_eq!(console.x_register, 42);
            assert_eq!(console.stack_pointer, 42);

            assert_eq!(cycles, 2);
        }

        #[test]
        fn test_tya_instruction() {

            let mut console = Console::new(Cartridge::new(vec![]));
            setup_instruction(&mut console, vec![0x_98]);

            console.accumulator = 0;
            console.y_register = 42;
            console.negative_flag = true;
            console.zero_flag = true;

            let cycles = execute_instruction(&mut console, tya_instruction);

            assert_eq!(console.accumulator, 42);
            assert_eq!(console.y_register, 42);
            assert_eq!(console.negative_flag, false);
            assert_eq!(console.zero_flag, false);

            assert_eq!(cycles, 2);
        }
}