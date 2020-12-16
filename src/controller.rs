// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of Atari 2600 Emulator which is released under the
// MIT license. Please refer to the LICENSE file that can be found at the root
// of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, September 2020

use crate::Console;

/// Brief description.
///
/// Long description.
///
pub trait Controller {
    fn plugged(&mut self, console: *mut Console);
    fn unplugged(&mut self);
}