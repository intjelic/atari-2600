// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of Atari 2600 Emulator which is released under the 
// MIT license. Please refer to the LICENSE file that can be found at the root 
// of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, September 2020

use std::time::Duration;

use crate::location::*;
use crate::instruction::*;

use crate::cartridge::Cartridge;
use crate::controller::Controller;
use crate::video::Video;
use crate::audio::Audio;

const CYCLE_DURATION: Duration = Duration::from_nanos(1_000_000_000 / 1_194_720);
// const CYCLE_DURATION: f32 = 1.0 / 1194720.0;

/// The TV type output.
/// 
/// The Atari 2600 gaming console has a physical switch to support black and 
/// white TV sets and colorful TV sets (NTSC and PAL only).
/// 
/// TODO; It's unclear to me if a color TV would be affected by the switch set 
/// to black and white; the description needs to be updated probably.
/// 
pub enum TvType {
    Mono, // 'W/B'
    Color // 'Colors'
}

/// The identification of the player.
/// 
/// The Atari 2600 gaming console supports up to 2 players denoted 'player 1' 
/// and 'player 2'.
/// 
pub enum Player {
    One, Two
}

/// The difficulty of the game for a given player.
/// 
/// The Atari 2600 gaming console has two physical switches to change the level 
/// of difficulty of player 1 and player 2. They're denoted 'amateur' for easy, 
/// and 'pro' for difficult.
/// 
pub enum Difficulty {
    Amateur, Pro
}

/// A virtual Atari 2600 gaming console.
/// 
/// This structure represents the physical Atari 2600 console. It's constructed 
/// with a mandatory cartridge, which can't be removed, and is turned on as 
/// soon as it's created and never turned off until the instance is destroyed. 
/// Therefore, there is no function to emulate changing the cartridge, and no 
/// function to emulate the physical power switch (on/off) that is present on 
/// a real console.
/// 
/// ```
/// let cartridge = Cartridge::open("breakout.bin");
/// let console = Console::new(cartridge);
/// ```
/// 
/// After the console instance is created, you must attach controller in either 
/// of the two ports. (explain how attaching/dettaching controllers works)
/// 
/// ```
/// // To be written.
/// ```
/// 
/// - explain how to advance the simulation in time
/// 
/// ```
/// // To be written.
/// ```
/// 
/// - explain how to retrieve the audio and video output
/// 
/// ```
/// // To be written.
/// ```
/// 
/// The console has **2 physical buttons** and **4 switches** which can also be changed
/// via some functions. There are the "reset" and "select" buttons, two switches 
/// to control the level of difficulty for each player, one switch to control 
/// the video output (monochrome/colorful) and a last one to turn on and off the 
/// console, but this last one doens't exist as the console is always turned on 
/// from the instant the it's created (limited by design). To turn it off in on,
/// you must re-create the console object. The following snippet shows how to 
/// use those functions.
/// 
/// ```
/// // Change the video output to become monochrome.
/// console.set_tv_type(TvType::Mono);
/// assert_eq!(console.tv_type(), TvType::Mono);
/// 
/// // Switch the difficulty of each players.
/// console.set_difficulty_switch(Player::One, Difficulty::Amateur);
/// console.set_difficulty_switch(Player::Two, Difficulty::Pro);
/// 
/// // Press the reset and select buttons.
/// console.reset_button();
/// console.select_button();
/// ```
/// 
/// Note that internally, the emulation of each hardware components (CPU, TIA, 
/// PIA, etc.) was all merged into this single structure as they are, on the 
/// hardware-level, very tied together. It would have been hard to split their 
/// implementation without overcomplicating the interface and the overall source
/// code of the emulator.
/// 
pub struct Console {
    // The pointer counter
    pub(crate) pointer_counter: u16,

    // The registers
    pub(crate) accumulator:  u8,
    pub(crate) x_register:  u8,
    pub(crate) y_register:  u8,

    // Teh status flags
    pub(crate) negative_flag: bool,
    pub(crate) overflow_flag: bool,
    pub(crate) break_flag: bool,
    pub(crate) decimal_flag: bool,
    pub(crate) interrupt_flag: bool,
    pub(crate) zero_flag: bool,
    pub(crate) carry_flag: bool,

    // The stack pointer
    pub(crate) stack_pointer: u8,

    // 0000-002C  TIA Write
    // 0000-000D  TIA Read (sometimes mirrored at 0030-003D)
    // 0080-00FF  PIA RAM (128 bytes)
    // 0280-0297  PIA Ports and Timer
    // F000-FFFF  Cartridge Memory (4 Kbytes area)

    // The memory (will change to intercept read/write)
    tia:   [u8; 62],  // from 0x_00 to 0x_3D
    ram:   [u8; 128], // from 0x_80 to 0x_FF
    riot:  [u8; 10],  // from 0x_0280 to 0x_0297
    // dummy: u8,        // for when the location isn't mapped to anything,
    memory: [u8; 8192],
    // pub(crate) memory: [u8; 8192], // 13-bit bus memory on 6507

    // // Number of cycles since the begining of the simulation.
    // cpu_cycles: u128,
    // tia_cycles: u128,

    elapsed_time: Duration,
    remaining_cycles: isize,

    cartridge: Cartridge,
    controller_left: Option<Box<dyn Controller>>,
    controller_right: Option<Box<dyn Controller>>,

    video: Video,
    audio: Audio,
}

impl Console {

    /// Create an Atari 2600 gaming console.
    /// 
    /// This function creates an Atari 2600 gaming console with a mandatory 
    /// cartridge which is never 'removed' during the emulation. To 'change' the 
    /// cartridge, you must create another console instance.
    /// 
    pub fn new(cartridge: Cartridge) -> Console {

        let mut console = Console {
            pointer_counter: 0,
            accumulator: 0,
            x_register: 0,
            y_register: 0,
            negative_flag: true,
            overflow_flag: true,
            break_flag: true,
            decimal_flag: true,
            interrupt_flag: true,
            zero_flag: true,
            carry_flag: true,
            // A well-behaving game will normally initialize the stack pointer.
            stack_pointer: 0x_FF, 

            tia:  [0; 62],
            ram:  [0; 128],
            riot: [0; 10],
            // dummy: 0,
            memory: [0; 8192],

            // cpu_cycles: u128,


            elapsed_time: Duration::new(0, 0),
            remaining_cycles: 0,

            cartridge: cartridge,

            controller_left: None,
            controller_right: None,
            // controllers: [Controller::new(), Controller::new()],
            video: Video::new(),
            audio: Audio::new()

        };

        console
    }

    /// Brief description.
    /// 
    /// Long description.
    /// 
    pub fn press_reset_button(&mut self) {
        *self.memory_mut(SWCHB) &= 0b1111_1110; // Bit 0 of SWCHB must be 0.
    }

    /// Brief description.
    /// 
    /// Long description.
    /// 
    pub fn release_reset_button(&mut self) {
        *self.memory_mut(SWCHB) |= 0b0000_0001; // Bit 0 of SWCHB must be 1.

    }

    /// Brief description.
    /// 
    /// Long description.
    /// 
    pub fn press_select_button(&mut self) {
        // Nothing to do; it's not controlled by the software and is not 
        // relevant in this context as we're not emulating a full-fledged TV 
        // set.
    }

    /// Brief description.
    /// 
    /// Long description.
    /// 
    pub fn release_select_button(&mut self) {
        // Nothing to do; it's not controlled by the software and is not 
        // relevant in this context as we're not emulating a full-fledged TV 
        // set.
    }

    /// Brief description.
    /// 
    /// Long description.
    /// 
    pub fn tv_type_switch(&self) -> TvType {

        match self.memory(SWCHB) & 0b0000_1000 > 0 {
            true  => TvType::Color,
            false => TvType::Mono
        }
    }

    /// Brief description.
    /// 
    /// Long description.
    /// 
    pub fn set_tv_type_switch(&mut self, tv_type: TvType) {
        // TODO; figure out what to do when it's SECAM, because the bit should 
        // always be 0.

        match tv_type {
            TvType::Color => *self.memory_mut(SWCHB) |= 0b0000_1000,
            TvType::Mono  => *self.memory_mut(SWCHB) &= 0b1111_0111
        }
    }

    /// Brief description.
    /// 
    /// Long description.
    /// 
    pub fn difficulty_switch(&self, player: Player) -> Difficulty {

        match player {
            Player::One => {
                match self.memory(SWCHB) & 0b0100_0000 > 0 {
                    true  => Difficulty::Pro,
                    false => Difficulty::Amateur
                }
            },
            Player::Two => {
                match self.memory(SWCHB) & 0b1000_0000 > 0 {
                    true  => Difficulty::Pro,
                    false => Difficulty::Amateur
                }
            }
        }
    }

    /// Brief description.
    /// 
    /// Long description.
    /// 
    pub fn set_difficulty_switch(&mut self, player: Player, difficulty: Difficulty) {

        match player {
            Player::One => {
                match difficulty {
                    Difficulty::Amateur => *self.memory_mut(SWCHB) |= 0b0100_0000,
                    Difficulty::Pro     => *self.memory_mut(SWCHB) &= 0b1011_1111
                }
            },
            Player::Two => {
                match difficulty {
                    Difficulty::Amateur => *self.memory_mut(SWCHB) |= 0b1000_0000,
                    Difficulty::Pro     => *self.memory_mut(SWCHB) &= 0b0111_1111
                }
            }
        }
    }

    /// Brief description.
    /// 
    /// Long description.
    /// 
    pub fn plug_controller(&mut self, slot: Player, mut controller: Box<dyn Controller>) {

        controller.plugged(&mut *self);

        match slot {
            Player::One => self.controller_left = Some(controller),
            Player::Two => self.controller_right = Some(controller)
        }
    }

    // pub fn unplug_controller(&mut self, slot: Player) -> dyn Controller {
        
    // }

    /// Advance the simulation in time.
    /// 
    /// This function must be called to advance the simulation in time. It's 
    /// called with the elapsed time which should be as small as possible to 
    /// avoid any weird 'time warp' effect.
    /// 
    /// Because nowadays CPUs run significantly faster than the console (about 
    /// 3000x faster), the time is adjusted to execute instructions at a slower 
    /// pace and match the execution speed of the console back then.
    /// 
    /// After this function is called, the audio and video components are 
    /// updated and can be used to display an eventual new TV frame or play the 
    /// sounds on your side.
    /// 
    pub fn update(&mut self, elapsed_time: Duration) {

        // // Update the global counters
        // self.cpu_cycles += 0;
        // self.tia_cycles += 0;


        // Update our own elapsed time tracker.
        self.elapsed_time += elapsed_time;

        // A division with remainder could have been used but it's not provided 
        // by the standard library, and it would likely result in poorer 
        // performance anyway as the elapsed time is usually very small.
        while self.elapsed_time >= CYCLE_DURATION {
            self.elapsed_time -= CYCLE_DURATION;
            self.remaining_cycles += 1;
        }

        // It's incovenient to compute how many cycles the next instruction will 
        // take, but at the same time, we can't be ahead of the simulation.
        // However, we know it will never exceeds 7 cycles, so we'll keep a 10 
        // cycles threshold.
        while self.remaining_cycles >= 10 {

            // execute instruction, get the cyc1les and substract

            // Right after 1 CPU cycle, 3 TIA cycles follows.
            self.video.execute_cycle();
            self.audio.execute_cycle();

            self.remaining_cycles -= 0;
        }

        // If remaining cycles was to be under 0, we'd be ahead of the 
        // simulation.
        assert!(self.remaining_cycles >= 0);
    }

    fn wait_for_leading_edge_of_horizontal_blank(&mut self) {}
    fn reset_horizontal_sync_counter(&mut self) {}

    fn reset_player_0(&mut self) {}
    fn reset_player_1(&mut self) {}
    fn reset_missile_0(&mut self) {}
    fn reset_missile_1(&mut self) {}
    fn reset_ball(&mut self) {}

    fn apply_horizontal_motion(&mut self) {}
    fn clear_horizontal_motion_registers(&mut self) {}
    fn clear_collision_latches(&mut self) {}
    
    pub(crate) fn memory<'a>(&self, mut index: u16) -> &'a u8 {
        // Cannot address more than 8192 bytes because bit 13, 14 and 15 are 
        // ignored on the MOS 6507 (bus lines aren't attached).
        index &= 0b0001_1111_1111_1111;

        let reference = match index {
            0x_00..=0x_3D => &self.tia[index as usize],
            0x_80..=0x_FF => &self.ram[(index - 0x_80) as usize],

            // The RIOT has 10 relevant memory locations and even if their 
            // address aren't contiguous, they're contained on a 10 bytes long 
            // array on this emulator.
            0x_0280..=0x_0285 => &self.riot[(index - 0x_0280) as usize],
            0x_0294..=0x_0297 => &self.riot[(6 + (index - 0x_0294)) as usize],

            // This portion of the memory is mapped to the ROM on the cartridge
            // but it's varying from cartridge to cartridge.
            0x_1000..=0x_1FFF => &self.cartridge.memory[(index - 0x_1000) as usize],

            // Adressing an irrelevant memory location, just returning 0; it's 
            // legal and it doesn't matter.
            //
            // TODO; Perhaps log this message, and also it could be a mapped 
            // memory which is not supported yet by this emulator.
            _ => &self.memory[index as usize]
            // _ => &self.dummy
        };

        unsafe {
            std::mem::transmute(reference)
        }
    }

    pub(crate) fn memory_mut<'a>(&mut self, mut index: u16) -> &'a mut u8 {

        // Cannot address more than 8192 bytes because bit 13, 14 and 15 are 
        // ignored on the MOS 6507 (bus lines aren't attached).
        index &= 0b0001_1111_1111_1111;

        let reference = match index {
            0x_00..=0x_3D => {
                match index {
                    0x_02 => self.wait_for_leading_edge_of_horizontal_blank(),
                    0x_03 => self.reset_horizontal_sync_counter(),
                    0x_10 => self.reset_player_0(),
                    0x_11 => self.reset_player_1(),
                    0x_12 => self.reset_missile_0(),
                    0x_13 => self.reset_missile_1(),
                    0x_14 => self.reset_ball(),
                    0x_2A => self.apply_horizontal_motion(),
                    0x_2B => self.clear_horizontal_motion_registers(),
                    0x_2C => self.clear_collision_latches(),
                    _ => ()
                }

                &mut self.tia[index as usize]
            },
            0x_80..=0x_FF => &mut self.ram[(index - 0x_80) as usize],

            // The RIOT has 10 relevant memory locations and even if their 
            // address aren't contiguous, they're contained on a 10 bytes long 
            // array on this emulator.
            0x_0280..=0x_0285 => &mut self.riot[(index - 0x_0280) as usize],
            0x_0294..=0x_0297 => &mut self.riot[(6 + (index - 0x_0294)) as usize],

            // This portion of the memory is mapped to the ROM on the cartridge
            // but it's varying from cartridge to cartridge.
            0x_1000..=0x_1FFF => &mut self.cartridge.memory[(index - 0x_1000) as usize],

            // Adressing an irrelevant memory location, just returning 0; it's 
            // legal and it doesn't matter.
            //
            // TODO; Perhaps log this message, and also it could be a mapped 
            // memory which is not supported yet by this emulator.
            _ => &mut self.memory[index as usize]
            // _ => &mut self.dummy
        };

        unsafe {
            std::mem::transmute(reference)
        }
    }

    /// Value pointed by the pointer counter.
    /// 
    /// This function returns the pointed value by the pointer counter (also 
    /// called the instruction pointer).
    /// 
    #[inline]
    pub(crate) fn pointed_value(&self) -> &u8 {
        &self.memory(self.pointer_counter)
    }

    /// Brief description.
    /// 
    /// This function does something that isn't documented yet.
    /// 
    #[inline]
    pub(crate) fn pointed_value_mut(&mut self) -> &mut u8 {
        self.memory_mut(self.pointer_counter)
    }

    /// Brief description.
    /// 
    /// This function does something that isn't documented yet.
    /// 
    #[inline]
    pub(crate) fn advance_pointer(&mut self) -> u8 {
        self.pointer_counter += 1;
        *self.memory(self.pointer_counter)
    }

    /// Brief description.
    /// 
    /// This function does something that isn't documented yet.
    /// 
    pub(crate) fn push_value(&mut self, value: u8) {
        // Stack is only 128 bytes long (merged with the RAM), if it were to 
        // go below, it would touch the TIA mapped registers. This would likely 
        // be a bug in the ROM.
        assert!(self.stack_pointer != 0x_79, "cannot push value; stack is full");

        *self.memory_mut(self.stack_pointer as u16) = value;
        self.stack_pointer -= 1;

    }

    /// Brief description.
    /// 
    /// This function does something that isn't documented yet.
    /// 
    pub(crate) fn pop_value(&mut self) -> u8 {
        assert!(self.stack_pointer != 0x_FF, "cannot pop value; stack is empty");

        self.stack_pointer += 1;
        *self.memory(self.stack_pointer as u16)
    }
    
    /// Execute the next instruction.
    /// 
    /// Long description to be written.
    /// 
    pub(crate) fn execute_instruction(&mut self) -> u32 {
        let opcode = *self.pointed_value();
        self.advance_pointer();

        let cycles = match opcode {
            0x_69 | 0x_65 | 0x_75 | 0x_6D | 0x_7D | 0x_79 | 0x_61 | 0x_71 => adc_instruction(self, opcode),
            0x_29 | 0x_25 | 0x_35 | 0x_2D | 0x_3D | 0x_39 | 0x_21 | 0x_31 => and_instruction(self, opcode),
            0x_0A | 0x_06 | 0x_16 | 0x_05 | 0x_1E => asl_instruction(self, opcode),
            0x_90 => bcc_instruction(self, opcode),
            0x_B0 => bcs_instruction(self, opcode),
            0x_F0 => beq_instruction(self, opcode),
            0x_24 | 0x_2C => bit_instruction(self, opcode),
            0x_30 => bmi_instruction(self, opcode),
            0x_D0 => bne_instruction(self, opcode),
            0x_10 => bpl_instruction(self, opcode),
            0x_00 => brk_instruction(self, opcode),
            0x_50 => bvc_instruction(self, opcode),
            0x_70 => bvs_instruction(self, opcode),
            0x_18 => clc_instruction(self, opcode),
            0x_D8 => cld_instruction(self, opcode),
            0x_58 => cli_instruction(self, opcode),
            0x_B8 => clv_instruction(self, opcode),
            0x_C9 | 0x_C5 | 0x_D5 | 0x_CD | 0x_DD | 0x_D9 | 0x_C1 | 0x_D1 => cmp_instruction(self, opcode),
            0x_E0 | 0x_E4 | 0x_EC => cpx_instruction(self, opcode),
            0x_C0 | 0x_C4 | 0x_CC => cpy_instruction(self, opcode),
            0x_C6 | 0x_D6 | 0x_CE | 0x_DE => dec_instruction(self, opcode),
            0x_CA => dex_instruction(self, opcode),
            0x_88 => dey_instruction(self, opcode),
            0x_49 | 0x_45 | 0x_55 | 0x_4D | 0x_5D | 0x_59 | 0x_41 | 0x_51 => eor_instruction(self, opcode),
            0x_E6 | 0x_F6 | 0x_EE | 0x_FE => inc_instruction(self, opcode),
            0x_E8 => inx_instruction(self, opcode),
            0x_C8 | 0x_42 => iny_instruction(self, opcode),
            0x_4C | 0x_6C => jmp_instruction(self, opcode),
            0x_20 => jsr_instruction(self, opcode),
            0x_A9 | 0x_A5 | 0x_B5 | 0x_AD | 0x_BD | 0x_B9 | 0x_A1 | 0x_B1 => lda_instruction(self, opcode),
            0x_A2 | 0x_A6 | 0x_B6 | 0x_AE | 0x_BE => ldx_instruction(self, opcode),
            0x_A0 | 0x_A4 | 0x_B4 | 0x_AC | 0x_BC => ldy_instruction(self, opcode),
            0x_4A | 0x_46 | 0x_56 | 0x_4E | 0x_5E => lsr_instruction(self, opcode),
            0x_EA => nop_instruction(self, opcode),
            0x_09 | 0x_05 | 0x_15 | 0x_0D | 0x_1D | 0x_19 | 0x_01 | 0x_11 => ora_instruction(self, opcode),
            0x_48 => pha_instruction(self, opcode),
            0x_08 => php_instruction(self, opcode),
            0x_68 => pla_instruction(self, opcode),
            0x_28 => plp_instruction(self, opcode),
            0x_2A | 0x_26 | 0x_36 | 0x_2E | 0x_3E => rol_instruction(self, opcode),
            0x_6A | 0x_66 | 0x_76 | 0x_6E | 0x_7E => ror_instruction(self, opcode),
            0x_40 => rti_instruction(self, opcode),
            0x_60 => rts_instruction(self, opcode),
            0x_E9 | 0x_E5 | 0x_F5 | 0x_ED | 0x_FD | 0x_F9 | 0x_E1 | 0x_F1 => sbc_instruction(self, opcode),
            0x_38 => sec_instruction(self, opcode),
            0x_F8 => sed_instruction(self, opcode),
            0x_78 => sei_instruction(self, opcode),
            0x_85 | 0x_95 | 0x_8D | 0x_9D | 0x_99 | 0x_81 | 0x_91 => sta_instruction(self, opcode),
            0x_89 | 0x_96 | 0x_8E => stx_instruction(self, opcode),
            0x_84 | 0x_94 | 0x_8C => sty_instruction(self, opcode),
            0x_AA => tax_instruction(self, opcode),
            0x_A8 => tay_instruction(self, opcode),
            0x_BA => tsx_instruction(self, opcode),
            0x_8A => txa_instruction(self, opcode),
            0x_9A => txs_instruction(self, opcode),
            0x_98 => tya_instruction(self, opcode),
            _ => panic!("unknown instruction")
        };

        cycles
    }
}