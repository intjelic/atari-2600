// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of Atari 2600 Emulator which is released under the 
// MIT license. Please refer to the LICENSE file that can be found at the root 
// of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, September 2020

use std::ptr;
use crate::console::Console;

/// The audio output.
/// 
/// This structure represents the audio output of the Atari 2600 gaming console.
/// 
/// TODO; To be implemented.
/// 
pub struct Audio {
    pub(crate) console: *mut Console
}

impl Audio {
    pub fn new() -> Audio {
        Audio {
            console: ptr::null_mut(),
        }
    }

    pub fn execute_cycle(&mut self) {
        // To be implemented.
    }
}