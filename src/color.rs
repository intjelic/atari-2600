// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of Atari 2600 Emulator which is released under the
// MIT license. Please refer to the LICENSE file that can be found at the root
// of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, September 2020

//! Color-related enumerations and helpers.
//!
//! This module defines the color enumerations for the **NTSC** TV sets and some
//! helpers to convert them into RGB colors. **PAL** and **SECAM** colors and
//! luminance are still to be implemented. Note that luminance is the same for
//! both NTSC and PAL and not used for SECAM.
//!
use crate::location::*;
use crate::console::Console;

/// Set of the luminance values as defined by the specifications (note that
/// the naming was made up).
pub enum Luminance {
    Darkest,
    VeryDark,
    Dark,
    SlightlyDark,
    SlightlyBright,
    Bright,
    VeryBright,
    Brightest,
}

/// Set of the NTSC color values as defined by the specifications (note two
/// values has the same name (blue) and this iis why one was renamed to
/// light blue).
pub enum Color {
    White,
    Gold,
    Orange,
    BrightOrange,
    Pink,
    Purple,
    PurpleBlue,
    Blue,
    Blue2,
    LightBlue,
    TorqueGreen,
    GreenBlue,
    Green,
    YellowGreen,
    OrangeGreen,
    LightOrange
}

/// Convert the luminance value to its enumeration counter-part (it's called
/// after the bits were extracted to form a value).
fn octal_to_luminance(value: u8) -> Luminance {
    match value {
        0 => Luminance::Darkest,
        1 => Luminance::VeryDark,
        2 => Luminance::Dark,
        3 => Luminance::SlightlyDark,
        4 => Luminance::SlightlyBright,
        5 => Luminance::Bright,
        6 => Luminance::VeryBright,
        7 => Luminance::Brightest,
        _ => panic!("luminance value must be an octal")
    }
}
/// Convert the color value to its enumeration counter-part (it's called after
/// the bits were extracted to form a value).
fn hexadecimal_to_color(value: u8) -> Color {
    match value {
        0  => Color::White,
        1  => Color::Gold,
        2  => Color::Orange,
        3  => Color::BrightOrange,
        4  => Color::Pink,
        5  => Color::Purple,
        6  => Color::PurpleBlue,
        7  => Color::Blue,
        8  => Color::Blue2,
        9  => Color::LightBlue,
        10 => Color::TorqueGreen,
        11 => Color::GreenBlue,
        12 => Color::Green,
        13 => Color::YellowGreen,
        14 => Color::OrangeGreen,
        15 => Color::LightOrange,
        _ => panic!("ntsc color value must be a hexadecimal")
    }
}

/// Dissect a byte and return the color and luminance information (they are
/// contained on a single byte; 3 bits for the luminance, and 4 bits for the
/// color).
fn color_and_luminance(value: u8) -> (Color, Luminance) {
    let color = (value & 0b11110000) >> 4;
    let luminance = (value & 0b00001110) >> 1;

    (hexadecimal_to_color(color), octal_to_luminance(luminance))
}

/// Compute the current background color determined by memory location COLUBK).
pub(crate) fn background_color(console: &Console) -> (u8, u8, u8) {
    to_rgb(color_and_luminance(*console.memory(COLUBK)))
}

/// Compute the current playfield color (determined by memory location COLUPF).
pub(crate) fn playfield_color(console: &Console) -> (u8, u8, u8) {
    to_rgb(color_and_luminance(*console.memory(COLUPF)))
}

/// Compute the current color of player 0 (determined by memory location
/// COLUP0).
pub(crate) fn player0_color(console: &Console) -> (u8, u8, u8) {
    to_rgb(color_and_luminance(*console.memory(COLUP0)))
}

/// Compute the current color of player 1 (determined by memory location
/// COLUP1).
pub(crate) fn player1_color(console: &Console) -> (u8, u8, u8) {
    to_rgb(color_and_luminance(*console.memory(COLUP1)))
}

/// Compute the current color of missile 0 (determined by memory location
/// COLUP0).
pub(crate) fn _missile0_color(console: &Console) -> (u8, u8, u8) {
    to_rgb(color_and_luminance(*console.memory(COLUP0)))
}

/// Compute the current color of missile 1 (determined by memory location
/// COLUP1).
pub(crate) fn _missile1_color(console: &Console) -> (u8, u8, u8) {
    to_rgb(color_and_luminance(*console.memory(COLUP1)))
}

/// Compute the current color of the ball (determined by memory location
/// COLUPF).
pub(crate) fn _ball_color(console: &Console) -> (u8, u8, u8) {
    to_rgb(color_and_luminance(*console.memory(COLUPF)))
}

/// Convert a color and a luminance into its corresponding RGB value to be
/// displayed on contemporary screen monitors.
pub fn to_rgb((color, luminance): (Color, Luminance)) -> (u8, u8, u8) {

    // Found on http://www.qotile.net/minidig/docs/tia_color.html
    match color {
        Color::White => {
            match luminance {
                Luminance::Darkest        => (0x00, 0x00, 0x00),
                Luminance::VeryDark       => (0x40, 0x40, 0x40),
                Luminance::Dark           => (0x6c, 0x6c, 0x6c),
                Luminance::SlightlyDark   => (0x90, 0x90, 0x90),
                Luminance::SlightlyBright => (0xb0, 0xb0, 0xb0),
                Luminance::Bright         => (0xc8, 0xc8, 0xc8),
                Luminance::VeryBright     => (0xdc, 0xdc, 0xdc),
                Luminance::Brightest      => (0xec, 0xec, 0xec),
            }
        },
        Color::Gold => {
            match luminance {
                Luminance::Darkest        => (0x44, 0x44, 0x00),
                Luminance::VeryDark       => (0x64, 0x64, 0x10),
                Luminance::Dark           => (0x84, 0x84, 0x24),
                Luminance::SlightlyDark   => (0xa0, 0xa0, 0x34),
                Luminance::SlightlyBright => (0xb8, 0xb8, 0x40),
                Luminance::Bright         => (0xd0, 0xd0, 0x50),
                Luminance::VeryBright     => (0xe8, 0xe8, 0x5c),
                Luminance::Brightest      => (0xfc, 0xfc, 0x68),
            }
        },
        Color::Orange => {

            match luminance {
                Luminance::Darkest        => (0x70, 0x28, 0x00),
                Luminance::VeryDark       => (0x84, 0x44, 0x14),
                Luminance::Dark           => (0x98, 0x5c, 0x28),
                Luminance::SlightlyDark   => (0xac, 0x78, 0x3c),
                Luminance::SlightlyBright => (0xbc, 0x8c, 0x4c),
                Luminance::Bright         => (0xcc, 0xa0, 0x5c),
                Luminance::VeryBright     => (0xdc, 0xb4, 0x68),
                Luminance::Brightest      => (0xec, 0xc8, 0x78),
            }
        },
        Color::BrightOrange => {
            match luminance {
                Luminance::Darkest        => (0x84, 0x18, 0x00),
                Luminance::VeryDark       => (0x98, 0x34, 0x18),
                Luminance::Dark           => (0xac, 0x50, 0x30),
                Luminance::SlightlyDark   => (0xc0, 0x68, 0x48),
                Luminance::SlightlyBright => (0xd0, 0x80, 0x5c),
                Luminance::Bright         => (0xe0, 0x94, 0x70),
                Luminance::VeryBright     => (0xec, 0xa8, 0x80),
                Luminance::Brightest      => (0xfc, 0xbc, 0x94),
            }
        },
        Color::Pink => {
            match luminance {
                Luminance::Darkest        => (0x84, 0x18, 0x00),
                Luminance::VeryDark       => (0x9c, 0x20, 0x20),
                Luminance::Dark           => (0xb0, 0x3c, 0x3c),
                Luminance::SlightlyDark   => (0xc0, 0x58, 0x58),
                Luminance::SlightlyBright => (0xd0, 0x70, 0x70),
                Luminance::Bright         => (0xe0, 0x88, 0x88),
                Luminance::VeryBright     => (0xec, 0xa0, 0xa0),
                Luminance::Brightest      => (0xfc, 0xb4, 0xb4),
            }
        },
        Color::Purple => {
            match luminance {
                Luminance::Darkest        => (0x78, 0x00, 0x5c),
                Luminance::VeryDark       => (0x8c, 0x20, 0x74),
                Luminance::Dark           => (0xa0, 0x3c, 0x88),
                Luminance::SlightlyDark   => (0x8c, 0x58, 0xb8),
                Luminance::SlightlyBright => (0xc0, 0x70, 0xb0),
                Luminance::Bright         => (0xd0, 0x84, 0xc0),
                Luminance::VeryBright     => (0xdc, 0x9c, 0xd0),
                Luminance::Brightest      => (0xec, 0xb0, 0xe0),
            }
        },
        Color::PurpleBlue => {
            match luminance {
                Luminance::Darkest        => (0x14, 0x00, 0x84),
                Luminance::VeryDark       => (0x30, 0x20, 0x98),
                Luminance::Dark           => (0x4c, 0x3c, 0xac),
                Luminance::SlightlyDark   => (0x68, 0x58, 0xc0),
                Luminance::SlightlyBright => (0x7c, 0x70, 0xd0),
                Luminance::Bright         => (0x94, 0x88, 0xe0),
                Luminance::VeryBright     => (0xa8, 0xa0, 0xec),
                Luminance::Brightest      => (0xbc, 0xb4, 0xfc),
            }
        },
        Color::Blue => {
            match luminance {
                Luminance::Darkest        => (0x00, 0x00, 0x88),
                Luminance::VeryDark       => (0x1c, 0x20, 0x9c),
                Luminance::Dark           => (0x38, 0x40, 0xb0),
                Luminance::SlightlyDark   => (0x50, 0x5c, 0xc0),
                Luminance::SlightlyBright => (0x68, 0x74, 0xd0),
                Luminance::Bright         => (0x7c, 0x8c, 0xe0),
                Luminance::VeryBright     => (0x90, 0xa4, 0xec),
                Luminance::Brightest      => (0xa4, 0xb8, 0xfc),
            }
        },
        Color::Blue2 => {
            match luminance {
                Luminance::Darkest        => (0x00, 0x18, 0x7c),
                Luminance::VeryDark       => (0x1c, 0x38, 0x90),
                Luminance::Dark           => (0x38, 0x54, 0xa8),
                Luminance::SlightlyDark   => (0x50, 0x70, 0xbc),
                Luminance::SlightlyBright => (0x68, 0x88, 0xcc),
                Luminance::Bright         => (0x7c, 0x9c, 0xdc),
                Luminance::VeryBright     => (0x90, 0xb4, 0xec),
                Luminance::Brightest      => (0xa4, 0xc8, 0xfc),
            }
        },
        // TODO; There must be a mistake somewhere around here.z
        Color::LightBlue => {
            match luminance {
                Luminance::Darkest        => (0x00, 0x18, 0x7c),
                Luminance::VeryDark       => (0x1c, 0x38, 0x90),
                Luminance::Dark           => (0x38, 0x54, 0xa8),
                Luminance::SlightlyDark   => (0x50, 0x70, 0xbc),
                Luminance::SlightlyBright => (0x68, 0x88, 0xcc),
                Luminance::Bright         => (0x7c, 0x9c, 0xdc),
                Luminance::VeryBright     => (0x90, 0xb4, 0xec),
                Luminance::Brightest      => (0xa4, 0xc8, 0xfc),
            }
        },
        Color::TorqueGreen => {
            match luminance {
                Luminance::Darkest        => (0x00, 0x2c, 0x5c),
                Luminance::VeryDark       => (0x1c, 0x4c, 0x78),
                Luminance::Dark           => (0x38, 0x68, 0x90),
                Luminance::SlightlyDark   => (0x50, 0x84, 0xac),
                Luminance::SlightlyBright => (0x68, 0x9c, 0xc0),
                Luminance::Bright         => (0x7c, 0xb4, 0xd4),
                Luminance::VeryBright     => (0x90, 0xcc, 0xe8),
                Luminance::Brightest      => (0xa4, 0xe0, 0xfc),
            }
        },
        Color::GreenBlue => {
            match luminance {
                Luminance::Darkest        => (0x00, 0x3c, 0x00),
                Luminance::VeryDark       => (0x20, 0x5c, 0x20),
                Luminance::Dark           => (0x40, 0x7c, 0x40),
                Luminance::SlightlyDark   => (0x5c, 0x9c, 0x5c),
                Luminance::SlightlyBright => (0x74, 0xb4, 0x74),
                Luminance::Bright         => (0x8c, 0xd0, 0x8c),
                Luminance::VeryBright     => (0xa4, 0xe4, 0xa4),
                Luminance::Brightest      => (0xb8, 0xfc, 0xb8),
            }
        },
        Color::Green => {
            match luminance {
                Luminance::Darkest        => (0x00, 0x3c, 0x2c),
                Luminance::VeryDark       => (0x1c, 0x5c, 0x48),
                Luminance::Dark           => (0x38, 0x7c, 0x64),
                Luminance::SlightlyDark   => (0x50, 0x9c, 0x80),
                Luminance::SlightlyBright => (0x68, 0xb4, 0x94),
                Luminance::Bright         => (0x7c, 0xd0, 0xac),
                Luminance::VeryBright     => (0x90, 0xe4, 0xc0),
                Luminance::Brightest      => (0xa4, 0xfc, 0xd4),
            }
        },
        Color::YellowGreen => {
            match luminance {
                Luminance::Darkest        => (0x14, 0x38, 0x00),
                Luminance::VeryDark       => (0x34, 0x5c, 0x1c),
                Luminance::Dark           => (0x50, 0x7c, 0x38),
                Luminance::SlightlyDark   => (0x6c, 0x98, 0x50),
                Luminance::SlightlyBright => (0x84, 0xb4, 0x68),
                Luminance::Bright         => (0x9c, 0xcc, 0x7c),
                Luminance::VeryBright     => (0xb4, 0xe4, 0x90),
                Luminance::Brightest      => (0xc8, 0xfc, 0xa4),
            }
        },
        Color::OrangeGreen => {
            match luminance {
                Luminance::Darkest        => (0x2c, 0x30, 0x000),
                Luminance::VeryDark       => (0x4c, 0x50, 0x1c),
                Luminance::Dark           => (0x68, 0x70, 0x34),
                Luminance::SlightlyDark   => (0x84, 0x8c, 0x4c),
                Luminance::SlightlyBright => (0x9c, 0xa8, 0x64),
                Luminance::Bright         => (0xb4, 0xc0, 0x78),
                Luminance::VeryBright     => (0xcc, 0xd4, 0x88),
                Luminance::Brightest      => (0xe0, 0xec, 0x9c),
            }
        },
        Color::LightOrange => {
            match luminance {
                Luminance::Darkest        => (0x44, 0x28, 0x00),
                Luminance::VeryDark       => (0x64, 0x48, 0x18),
                Luminance::Dark           => (0x84, 0x68, 0x30),
                Luminance::SlightlyDark   => (0xa0, 0x84, 0x44),
                Luminance::SlightlyBright => (0xb8, 0x9c, 0x58),
                Luminance::Bright         => (0xd0, 0xb4, 0x6c),
                Luminance::VeryBright     => (0xe8, 0xcc, 0x7c),
                Luminance::Brightest      => (0xfc, 0xe0, 0x8c),
            }
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_color_and_luminance() {
        // TODO; To be implemented.

        // assert_eq!(
        //     color_and_luminance(0b01010101),
        //     (Color::Purple, Luminance::Dark)
        // );
        // assert_eq!(
        //     color_and_luminance(0b10101010),
        //     (Color::TorqueGreen, Luminance::Bright)
        // );
        // assert_eq!(
        //     color_and_luminance(0b00001111),
        //     (Color::White, Luminance::Brightest)
        // );
        // assert_eq!(
        //     color_and_luminance(0b11110000),
        //     (Color::LightOrange, Luminance::Darkest)
        // );
    }

    #[test]
    fn test_color_to_rgb() {
        // TODO; To be implemented.
    }
}