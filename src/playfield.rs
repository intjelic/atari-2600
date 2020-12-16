
// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of Atari 2600 Emulator which is released under the
// MIT license. Please refer to the LICENSE file that can be found at the root
// of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, November 2020

//! Brief description.
//!
//! This module defines something that is to be described.
//!
//! TODO; Write description of this module.
//!
use crate::location::{PF0, PF1, PF2, CTRLPF};
use crate::console::Console;
use crate::utils::byte_to_boolean_array;

pub(crate) fn playfield_mirror_mode(console: &Console) -> bool {
    *console.memory(CTRLPF) & 0b000_0001 != 0
}

pub(crate) fn playfield_priority(console: &Console) -> bool {
    *console.memory(CTRLPF) & 0b0000_0100 != 0
}

pub(crate) fn playfield_color(console: &Console) -> (u8, u8, u8) {
    crate::color::playfield_color(console)
}

pub(crate) fn playfield_left_color(console: &Console) -> (u8, u8, u8) {
    crate::color::player0_color(console)
}

pub(crate) fn playfield_right_color(console: &Console) -> (u8, u8, u8) {
    crate::color::player1_color(console)
}

pub(crate) fn playfield_score_mode(console: &Console) -> bool {
    *console.memory(CTRLPF) & 0b0000_0010 != 0
}

pub(crate) fn playfield_bits(console: &Console) -> [bool; 20] {
    let pf0_bits = byte_to_boolean_array(*console.memory(PF0));
    let pf1_bits = byte_to_boolean_array(*console.memory(PF1));
    let pf2_bits = byte_to_boolean_array(*console.memory(PF2));

    [
        pf0_bits[4],
        pf0_bits[5],
        pf0_bits[6],
        pf0_bits[7],

        pf1_bits[0],
        pf1_bits[1],
        pf1_bits[2],
        pf1_bits[3],
        pf1_bits[4],
        pf1_bits[5],
        pf1_bits[6],
        pf1_bits[7],

        pf2_bits[0],
        pf2_bits[1],
        pf2_bits[2],
        pf2_bits[3],
        pf2_bits[4],
        pf2_bits[5],
        pf2_bits[6],
        pf2_bits[7],
    ]
}

#[cfg(test)]
mod test {

    #[test]
    fn test_playfield() {
    }
}