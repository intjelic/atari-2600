
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
use crate::location::{ENAM0, ENAM1};
use crate::console::Console;

fn _is_missile0_enabled(console: &Console) -> bool {
    //   1D      ENAM0   ......1.  graphics (enable) missile 0
    *console.memory(ENAM0) & 0b0000_00010 > 0
}

fn _is_missile1_enabled(console: &Console) -> bool {
    //   1E      ENAM1   ......1.  graphics (enable) missile 1
    *console.memory(ENAM1) & 0b0000_00010 > 0
}


#[cfg(test)]
mod test {

    #[test]
    fn test_is_missile_enabled() {
        // assert_eq!(is_missile0_enabled(console: &Console))
    }
}