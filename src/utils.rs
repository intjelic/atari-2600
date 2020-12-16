// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of Atari 2600 Emulator which is released under the
// MIT license. Please refer to the LICENSE file that can be found at the root
// of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, December 2020

pub(crate) fn byte_to_boolean_array(value: u8) -> [bool; 8] {
    [
        value & 0b00000001 != 0,
        value & 0b00000010 != 0,
        value & 0b00000100 != 0,
        value & 0b00001000 != 0,
        value & 0b00010000 != 0,
        value & 0b00100000 != 0,
        value & 0b01000000 != 0,
        value & 0b10000000 != 0
    ]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_byte_to_boolean_array() {
        assert_eq!(
            byte_to_boolean_array(0b01010101),
            [true, false, true, false, true, false, true, false]
        );
        assert_eq!(
            byte_to_boolean_array(0b10101010),
            [false, true, false, true, false, true, false, true]
        );
        assert_eq!(
            byte_to_boolean_array(0b00001111),
            [true, true, true, true, false, false, false, false]
        );
        assert_eq!(
            byte_to_boolean_array(0b11110000),
            [false, false, false, false, true, true, true, true]
        );
    }
}