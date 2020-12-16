// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of Atari 2600 Emulator which is released under the
// MIT license. Please refer to the LICENSE file that can be found at the root
// of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, December 2020

//! Brief description.
//!
//! This module defines something that is to be described.
//!
use crate::location::{GRP0, GRP1, REFP0, REFP1};
use crate::console::Console;
use crate::console::Player;
use crate::utils::byte_to_boolean_array;

pub(crate) fn _player_bits(console: &Console, player: Player) -> [bool; 8] {
    match player {
        Player::One => byte_to_boolean_array(*console.memory(GRP0)),
        Player::Two => byte_to_boolean_array(*console.memory(GRP1))
    }
}

pub(crate) fn _is_player_mirrored(console: &Console, player: Player) -> bool {
    match player {
        Player::One => *console.memory(REFP0) & 0b000_1000 != 0,
        Player::Two => *console.memory(REFP1) & 0b000_1000 != 0
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_player() {
    }
}