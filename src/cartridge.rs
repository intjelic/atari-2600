// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of Atari 2600 Emulator which is released under the 
// MIT license. Please refer to the LICENSE file that can be found at the root 
// of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, September 2020

use std::io;
use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::string::String;

/// Game cartridge of the Atari 2600 gaming console.
/// 
/// A cartridge contains up to 4k ROm which is mapped to the RAM from 0x_1000 to 
/// 0x_1FFF. It contains metadata such as X, Y.
/// 
/// TODO; To be implemented.
/// 
/// Pending notes:
/// --------------
/// - if the rom is less than 4k, the entire reserved memory isn't filled up
/// - memory also ROM, or EPROM
/// 
pub struct Cartridge {
    pub name: String,
    pub manufacturer: String,
    pub model: String,
    pub rarity: String,
    pub notes: String,
    pub memory: Vec<u8>
}

impl Cartridge {
    pub fn new(memory: Vec<u8>) -> Cartridge {
        Cartridge {
            name: String::new(),
            manufacturer: String::new(),
            model: String::new(),
            rarity: String::new(),
            notes: String::new(),
            memory: memory
        }
    }

    pub fn from_reader<R: Read>(reader: &mut R) -> io::Result<Cartridge> {
        let bytes = Vec::new();
        // reader.read_to_end(&mut bytes)?;

        // TODO; To be implemented.
    
        Ok(Cartridge::new(bytes))
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Cartridge> {
        let mut reader = File::open(path)?;
        Self::from_reader(&mut reader)
    }
}