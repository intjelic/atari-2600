// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of Atari 2600 Emulator which is released under the
// MIT license. Please refer to the LICENSE file that can be found at the root
// of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, September 2020

//! Emulator of the Atari 2600 gaming console.
//!
//! This crate contains the various components needed to accurately emulate any
//! Atari 2600 games, as well as a ready-to-use emulator launchable from the
//! terminal; except a window to display the output of the game, no additional
//! graphical user interface was implemented. Note that unlike most emulators,
//! it excludes the 7800 version of the console and only supports NTSC video
//! output. Overall, the code is fairly well-documented and you should be able
//! to get around easily.
//!
//! **Features**
//!
//! - NTSC, PAL and SECAM TV sets
//! - All official controllers
//!
//! # Get started
//!
//! Naming
//!
//! It was hard to dissect the different components (CPU, RAM, PIA, TIA, etc.)
//! without making the architecture too heavy and unnecesiryly complicated, so
//! instead, it just revolves around the `Console` structure which is the very
//! main component.
//!
//! ```
//! let cartridge = Cartridge::open("breakout.bin");
//! let console = Console::new(cartridge);
//!
//! console.update(elapsed_time);
//!
//! display_frame(console.video.output);
//! play_samples(console.audio.output);
//!
//! console.controllers[0].press_button();
//! ```
//!
//! It represents a virtual gaming console with .
//! Cartridge can't be removed during the simulation. Two controllers are always
//! plugged in, and TV set is NTSC and plugged too.
//!
//! ```
//! ```
//!
//! Normally cartridges are created from the ROM binary file.
//!
//! represents the hardware of the
//! console itself. See it as
//!
//! # More information
//!
//! For specifications and more information about the gaming console, look at
//! the following.
//!
//! - https://problemkaputt.de/2k6specs.htm
//! - foo
//! - bar
//!
//! Useful documents were also added directly to the source repository.
//!
pub(crate) mod location;
pub mod addressing_mode;
pub mod instruction;
pub(crate) mod color;
pub(crate) mod playfield;
pub(crate) mod sprite;
pub(crate) mod missile;
pub(crate) mod ball;
pub(crate) mod utils;

mod cartridge;
mod controller;
mod joystick;
mod paddle;
mod keypad;
mod steering;
mod lightgun;
mod trackball;
mod video;
mod audio;
mod console;
mod emulator;

pub use cartridge::Cartridge;
pub use controller::Controller;
pub use joystick::Joystick;
pub use paddle::Paddle;
pub use keypad::Keypad;
pub use steering::Steering;
pub use lightgun::Lightgun;
pub use trackball::Trackball;
pub use console::{TvType, Player, Difficulty};
pub use console::Console;
pub use emulator::Emulator;