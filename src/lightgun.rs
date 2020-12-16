// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of Atari 2600 Emulator which is released under the
// MIT license. Please refer to the LICENSE file that can be found at the root
// of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, November 2020

use crate::Console;
use crate::Controller;

/// Brief description.
///
/// Long description.
///
pub struct Lightgun {
    console: Option<*mut Console>
}

impl Lightgun {
}

impl Controller for Lightgun {
    fn plugged(&mut self, console: *mut Console) {
        self.console = Some(console);
    }

    fn unplugged(&mut self) {
        self.console = None;
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_lightgun() {
    }
}
