
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
use crate::location::ENABL;
use crate::console::Console;

enum BallSize {
    One,
    Two,
    Four,
    Eight
}

fn _is_ball_enabled(console: &Console) -> bool {
    //   1F      ENABL   ......1.  graphics (enable) ball
    *console.memory(ENABL) & 0b0000_00010 > 0
}

// fn ball_size(console: &Console) -> BallSize {
//     // 0Ah - CTRLPF - Control Playfield and Ball size

//     // Bit  Expl.
//     // 0    Playfield Reflection     (0=Normal, 1=Mirror right half)
//     // 1    Playfield Color          (0=Normal, 1=Score Mode, only if Bit2=0)
//     // 2    Playfield/Ball Priority  (0=Normal, 1=Above Players/Missiles)
//     // 3    Not used
//     // 4-5  Ball size                (0..3 = 1,2,4,8 pixels width)
//     // 6-7  Not used
//     let value = *console.memory(CTRLPF) & 0b0011_0000 >> 4;

//     match value {
//         0 => BallSize::One,
//         1 => BallSize::Two,
//         2 => BallSize::Four,
//         3 => BallSize::Eight
//     }
// }

#[cfg(test)]
mod test {

    #[test]
    fn test_ball() {
        // TODO; To be implemented.
    }

    // #[test]
    // fn test_is_ball_enabled() {
    //     // assert_eq!(is_missile0_enabled(console: &Console))
    // }

    // #[test]
    // fn test_ball_size() {
    //     assert_eq!(ball_size(0b00010101), 1);
    //     assert_eq!(ball_size(0b10101010), 2);
    //     assert_eq!(ball_size(0b01001111), 4);
    //     assert_eq!(ball_size(0b11110000), 8);
    // }

}