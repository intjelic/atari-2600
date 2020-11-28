// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of Atari 2600 Emulator which is released under the 
// MIT license. Please refer to the LICENSE file that can be found at the root 
// of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, September 2020

//! The MOS 6507 addressing modes.
//!
//! This module contains helper functions to compute the operand on which 
//! instructions operate. Unless the operand is implied, each 6507 instructions 
//! will define an address mode to compute the associated operand.
//! 
//! The addressing modes:
//! 
//! - Implied (not technically an addressing mode)
//! - Relative
//! - Immediate
//! - Zero page (X & Y)
//! - Absolute (X & Y)
//! - Indexed Indirect
//! - Indirect Index
//! 
//! Note that they're tightly coupled with the instructions and there is no unit 
//! tests as they're indirectly tested with the instructions unit tests.
//! 
use super::console::Console;

/// Relative addressing mode.
/// 
/// The relative addressing mode designates the operand as a value in memory 
/// indexed by the pointer counter.
/// 
/// This function consumes the relevant bytes following the opcode and returns 
/// the index of the value in memory on which the instruction must operate. 
/// 
pub fn relative(console: &mut Console) -> i8 {
    let index = console.pointer_counter;
    console.advance_pointer(); 

    *console.memory(index) as i8
}

/// Immediate addressing mode.
/// 
/// The immediate addressing mode designates the operand as the immediate byte 
/// following the opcode. 
/// 
/// This function consumes the relevant bytes following the opcode and returns 
/// the index of the value in memory on which the instruction must operate. 
///
pub fn immediate(console: &mut Console) -> u16 {
    let index = console.pointer_counter;
    console.advance_pointer(); 

    index
}

/// Zero page addressing mode.
/// 
/// The zero page addressing mode designates the operand as a value in the first 
/// 256 bytes of the memory which is indexed by the immediate byte following the 
/// opcode.
/// 
/// This function consumes the relevant bytes following the opcode and returns 
/// the index of the value in memory on which the instruction must operate. 
/// 
pub fn zero_page(console: &mut Console) -> u16 {
    let index = *console.pointed_value() as u16;
    console.advance_pointer();

    index
}

/// Zero page X addressing mode.
/// 
/// The zero page X address mode designates the operand as a value in the first 
/// 256 bytes of the memory which is indexed by the immediate byte following the 
/// opcode plus the value of the X register.
/// 
/// This function consumes the relevant bytes following the opcode and returns 
/// the index of the value in memory on which the instruction must operate. 
/// 
pub fn zero_page_x(console: &mut Console) -> u16 {
    let index = console.pointed_value().wrapping_add(console.x_register) as u16;
    console.advance_pointer();

    index
}

/// Zero page Y address mode.
/// 
/// The zero page Y address mode designates the operand as a value in the first 
/// 256 bytes of the memory which is indexed by the immediate byte following the 
/// opcode plus the value of the Y register.
/// 
/// This function consumes the relevant bytes following the opcode and returns 
/// the index of the value in memory on which the instruction must operate. 
/// 
pub fn zero_page_y(console: &mut Console) -> u16 {
    let index = console.pointed_value().wrapping_add(console.y_register) as u16;
    console.advance_pointer();

    index
}

/// Absolute addressing mode.
/// 
/// The zero page addressing mode designates the operand as a a value anywhere 
/// in the memory, which is indexed by the two following bytes. The first byte 
/// corresponds to the page number and the second byte is the index on this the 
/// page.
/// 
/// This function consumes the relevant bytes following the opcode and returns 
/// the index of the value in memory on which the instruction must operate. 
/// 
pub fn absolute(console: &mut Console) -> u16 {
    let ll = *console.pointed_value();
    console.advance_pointer();
    let hh = *console.pointed_value();
    console.advance_pointer();
    
    u16::from_le_bytes([ll, hh])
}

/// Absolute X addressing mode.
/// 
/// The zero page X addressing mode designates the operand as a a value anywhere 
/// in the memory, which is indexed by the two following bytes and the X 
/// register. The first byte corresponds to the page number and the second byte 
/// **plus** the X register value is the index on this the page. If the addition 
/// of the second byte with the X register value overflows, the page number is 
/// incremented and most instructions will add a cycle; this is why a boolean 
/// value is returned.
/// 
/// This function consumes the relevant bytes following the opcode and returns 
/// the index of the value in memory on which the instruction must operate. 
/// 
pub fn absolute_x(console: &mut Console) -> (u16, bool) {
    let ll = *console.pointed_value();
    console.advance_pointer();
    let hh = *console.pointed_value();
    console.advance_pointer();
    
    match ll.overflowing_add(console.x_register) {
        (value, false) => (u16::from_le_bytes([value, hh]), false),
        (value, true) => {
            (u16::from_le_bytes([value, hh.wrapping_add(1)]), true)
        }
    }
} 

/// Absolute Y addressing mode.
/// 
/// The zero page Y addressing mode designates the operand as a a value anywhere 
/// in the memory, which is indexed by the two following bytes and the Y 
/// register. The first byte corresponds to the page number and the second byte 
/// **plus** the Y register value is the index on this the page. If the addition 
/// of the second byte with the Y register value overflows, the page number is 
/// incremented and most instructions will add a cycle; this is why a boolean 
/// value is returned.
/// 
/// This function consumes the relevant bytes following the opcode and returns 
/// the index of the value in memory on which the instruction must operate. 
/// 
pub fn absolute_y(console: &mut Console) -> (u16, bool) {
    let ll = *console.pointed_value();
    console.advance_pointer();
    let hh = *console.pointed_value();
    console.advance_pointer();
    
    match ll.overflowing_add(console.y_register) {
        (value, false) => (u16::from_le_bytes([value, hh]), false),
        (value, true) => {
            (u16::from_le_bytes([value, hh.wrapping_add(1)]), true)
        }
    }
} 

/// Indexed indirect addressing mode.
/// 
/// The indexed indirect addressing mode designates the operand as foobar.
/// 
/// ```
/// INDEXED  INDIRECT ADDRESSING  -  In  indexed  indirect  addressing  (referred   to   as  (Indirect,X)), the  second byte  ofthe  instruction  is  added  to  the  contents  of  the  X  index  register,  discarding  the  carry.   The  result of  this  addition  points  to a memory  location  on page  zero whose  contents  is  the  low order  eight  bits of  the  effective  address.   The  next  memory  location  in  page  zero  contains  the high  order  eight  bits of  the  effective  address.   Both memory  locations  specifying  the  high  and  low order  bytes  of  the effective  address must  be  in  page  zero.
/// ```
/// 
/// TODO; To be written.
/// 
pub fn indexed_indirect(console: &mut Console) -> u16 {
    let indirect_index = console.pointed_value().wrapping_add(console.x_register);
    console.advance_pointer();

    let ll = *console.memory(indirect_index as u16);
    // TODO; Make sure indirect_index + 1 is whitng page 0, otherwise it's illegal operation I think.
    let hh = *console.memory(indirect_index as u16 + 1);

    let index = u16::from_le_bytes([ll, hh]);

    index
}


/// Indirect indexed addressing mode.
/// 
/// The indirect indexed addressing mode designates the operand as foobar.
/// 
/// ```
/// INDIRECT  INDEXED ADDRESSING  -  In  indirect  indexed  addressing  (referred  to  as (Indirect),Y),  the    second  byteof  the  instruction  points  to  a memory  location  in  page  zero.   The  contents  of  this memory  location is  added  to  the  contents  of  the  Y  index  register,  the  result  being the  low order eight  bits  of  theeffective  address.   The  carry  from  this  addition  is  added  to  the  contents  of  the next     page  zeromemory  location,  the  result  being  the  high  order  eight  bits  of  the  effective  address.
/// ```
/// 
/// TODO; To be written.
/// 
pub fn indirect_indexed(console: &mut Console) -> (u16, bool) {
    let operand = *console.pointed_value();
    console.advance_pointer();

    let indirect_index = console.memory(operand as u16).wrapping_add(console.y_register);

    let ll = *console.memory(indirect_index as u16);
    let hh = *console.memory(indirect_index as u16 + 1);

    let index = u16::from_le_bytes([ll, hh]);

    (index, false)
}