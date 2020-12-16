// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of Atari 2600 Emulator which is released under the
// MIT license. Please refer to the LICENSE file that can be found at the root
// of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, September 2020

//! Video-related enumerations and helpers.
//!
//! TODO; Write the description.
//!
use crate::color::{background_color};
use crate::playfield::{
    playfield_mirror_mode,
    playfield_priority,
    playfield_color, playfield_left_color, playfield_right_color,
    playfield_score_mode,
    playfield_bits
};
use crate::console::Console;

fn draw_playfield(console: &Console, scanline: &mut [(u8, u8, u8); 160]) {
    // The playfield can be drawn above or under the other objects, but it's not
    // the responsibility of this function (it's the responsibility of the
    // caller).

    // Basically, there are 2x2 modes which are independent and thus resulting
    // in 4 different code paths.
    // If the "score mode" is activated, the color used to draw the playfield
    // becomes the color of player 1 & 2, where color of player 1 will be used
    // to draw the left side of the playfield, and color of player 2 will be
    // used to draw the right side.
    // If the "mirror mode" is used, the right side of the playfield becomes
    // the left side flipped horizontally.
    let score_mode = playfield_score_mode(console);
    let mirror_mode = playfield_mirror_mode(console);

    // We retrieve the data of the playfield (the bits that defines whether
    // the playfield is display on some pixels or not).
    let bits = playfield_bits(console);

    // Draw the left side of the playground.
    let color = match score_mode {
        true  => playfield_left_color(console),
        false => playfield_color(console)
    };

    for (index, bit) in bits.iter().enumerate() {
        if *bit {
            scanline[index * 4 + 0] = color;
            scanline[index * 4 + 1] = color;
            scanline[index * 4 + 2] = color;
            scanline[index * 4 + 3] = color;
        }
    }

    // Draw the right side of the playground.
    let color = match score_mode {
        true  => playfield_right_color(console),
        false => playfield_color(console)
    };

    if mirror_mode {
        for (index, bit) in bits.iter().rev().enumerate() {
            if *bit {
                scanline[80 + index * 4 + 0] = color;
                scanline[80 + index * 4 + 1] = color;
                scanline[80 + index * 4 + 2] = color;
                scanline[80 + index * 4 + 3] = color;
            }
        }
    } else {
        for (index, bit) in bits.iter().enumerate() {
            if *bit {
                scanline[80 + index * 4 + 0] = color;
                scanline[80 + index * 4 + 1] = color;
                scanline[80 + index * 4 + 2] = color;
                scanline[80 + index * 4 + 3] = color;
            }
        }
    }
}

fn draw_sprites(_console: &Console, _scanline: &mut [(u8, u8, u8); 160]) {
    // TODO; To be implemented.
}

fn draw_missiles(_console: &Console, _scanline: &mut [(u8, u8, u8); 160]) {
    // TODO; To be implemented.
}

fn draw_ball(_console: &Console, _scanline: &mut [(u8, u8, u8); 160]) {
    // TODO; To be implemented.
}

pub(crate) fn create_scanline(console: &Console) -> [(u8, u8, u8); 160] {

    // First, create and fill the entire scanline with the background color.
    let background_colorr = background_color(console);
    let mut scanline = [background_colorr; 160];

    let playfield_priority = playfield_priority(console);

    if playfield_priority {
        draw_playfield(console, &mut scanline);
        draw_sprites(console, &mut scanline);
        draw_missiles(console, &mut scanline);
        draw_ball(console, &mut scanline);
    }
    else {
        draw_sprites(console, &mut scanline);
        draw_missiles(console, &mut scanline);
        draw_ball(console, &mut scanline);
        draw_playfield(console, &mut scanline);
    }

    scanline
}

#[cfg(test)]
mod test {

    #[test]
    fn test_video() {
    }
}