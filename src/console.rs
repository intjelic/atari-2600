// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of Atari 2600 Emulator which is released under the
// MIT license. Please refer to the LICENSE file that can be found at the root
// of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, September 2020

use std::time::Duration;

use crate::cartridge::Cartridge;
use crate::controller::Controller;
use crate::location::*;
use crate::location::{VSYNC};
use crate::instruction::*;
use crate::video::create_scanline;

const HORIZONTAL_CYCLES: u32 = 228;
const VERTICAL_LINES: u32 = 262;

// TODO; Double-check exact cycle duration because TV runs at 59.94 Hertz, not
// exactly 60 Hertz, therefore 228 * 262 / 3 * 59.94 results in a bit less than
// the current number below.

// const CYCLE_DURATION: Duration = Duration::from_nanos(1_000_000_000 / 1_194_720);
const CYCLE_DURATION: Duration = Duration::from_nanos(1_000_000_000 / 1_193_525);

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
    tia: [u8; 62],  // from 0x_00 to 0x_3D
    ram: [u8; 128], // from 0x_80 to 0x_FF
    pia: [u8; 4],   // from 0x_0280 to 0x_0297 but timer-related values were taken out.

    // dummy: u8,        // for when the location isn't mapped to anything,
    dummy: [u8; 8192],
    // pub(crate) memory: [u8; 8192], // 13-bit bus memory on 6507

    // Timer-related values from the PIA.
    timer_value: u8,
    timer_status: u8, // only bit 7 and 6 are relevant
    timer_interval: u32,
    timer_elapsed_clocks: u32,

    // Number of cycles since the beginning of the simulation.
    cycles_count: u128,
    color_cycles_count: u128,
    instructions_count: u128,

    players_position: [u32; 2],
    missiles_position: [u32; 2],
    ball_position: u32,

    scanline: u32,
    scanline_cycle: u32,

    is_vsync: bool,
    cpu_halt: bool,

    pub framebuffer: [[(u8, u8, u8); 160]; 192],
    pending_framebuffer: [[(u8, u8, u8); 160]; 192],


    // Simulation timing variables.
    elapsed_time: Duration,  // Local elapsed time
    remaining_cycles: isize, //
    timer_block: bool, // tmp

    cartridge: Cartridge,
    controller_left: Option<Box<dyn Controller>>,
    controller_right: Option<Box<dyn Controller>>
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
            pointer_counter: 0x_F000, // TODO; double-check this
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

            tia: [0; 62],
            ram: [0; 128],
            pia: [0; 4],
            // dummy: 0,
            dummy: [0; 8192],

            timer_value: 0,
            timer_status: 0,
            timer_interval: 1,
            timer_elapsed_clocks: 1,

            cycles_count: 0,
            color_cycles_count: 0,
            instructions_count: 0,

            players_position: [0; 2],
            missiles_position: [0; 2],
            ball_position: 0,

            scanline: 0,
            scanline_cycle: 0,

            is_vsync: false,
            cpu_halt: false,

            framebuffer: [[(0, 0, 0); 160]; 192],
            pending_framebuffer: [[(0, 0, 0); 160]; 192],

            elapsed_time: Duration::new(0, 0),
            remaining_cycles: 0,
            timer_block: true,

            cartridge: cartridge,

            controller_left: None,
            controller_right: None,
            // controllers: [Controller::new(), Controller::new()],
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

    fn is_horizontal_blank(&self) -> bool {
        self.scanline_cycle < 68
    }

    fn is_vertical_sync(&self) -> bool {
        self.scanline < 3
    }

    fn is_vertical_blank(&self) -> bool {
        self.scanline >= 3 && self.scanline < 3 + 37
    }

    fn is_overscan(&self) -> bool {
        self.scanline >= 3 + 37 + 192
    }

    fn is_beam_drawing(&self) -> bool {

        // todo; rename this function
        let a = self.scanline >= 3 + 37 && self.scanline < 3 + 37 + 192;
        let b = !self.is_horizontal_blank();

        a && b
    }

    fn beam_position(&self) -> (usize, usize) { // return current normalized line and "pixel"

        assert!(self.is_beam_drawing());

        let line = self.scanline - (3 + 37);
        let pixel = self.scanline_cycle - 68;

        (line as usize, pixel as usize)
    }

    pub fn update_timer(&mut self) {


        // When the elapsed clocks variable reaches 0, we must decrement the
        // timer value.
        self.timer_elapsed_clocks -= 1;
        if self.timer_elapsed_clocks == 0 {

            // If the timer value is 0, it's underflowing and we must update the
            // timer status (bit 6 and 7).
            if self.timer_value == 0 {

                // The timer value reached 0, the timer is now entering the
                // high speed decrement mode.
                self.timer_interval = 1;

                // Update the timer status.
                self.timer_status |= 0b_1100_0000;
            }

            // Decrement the timer value.
            self.timer_value = self.timer_value.wrapping_sub(1);

            // Adjust the elapsed clocks according to the current timer
            // interval.
            self.timer_elapsed_clocks = self.timer_interval;
        }


    }
    pub fn execute_cycle(&mut self) {


        // Update the timer unless it's 'blocked'. It's a little hack that we
        // are forced to introduce because it would be inconvenient to know in
        // advance how many cycles an instruction would take. We must not update
        // the timer during the cycles that an instruction modifying the timer
        // register is taking, otherwise the timer would be decrement
        // prematurely.
        if !self.timer_block {
            self.update_timer();
        }

        // Check for change in the VSYNC bit and adjust scanline accordingly if
        // it was switched off.
        let vsync_bit = *self.memory(VSYNC) & 0b_0000_0010 > 0;
        if self.is_vsync && vsync_bit == false { // Check for vsync being switched off
            self.scanline = 2;
        }
        self.is_vsync = vsync_bit;

        self.execute_color_cycle();
        self.execute_color_cycle();
        self.execute_color_cycle();

        // Update cycles counters (for debugging and analysis).
        self.cycles_count += 1;
        self.color_cycles_count += 3;
    }
    pub fn execute_color_cycle(&mut self) {

        // // Draw the current pixel if the beam is on a drawable area.
        // if self.is_beam_drawing() {
        //     let (line, pixel) = self.beam_position();
        //     println!("drawing at {}, {}", line, pixel);

        //     self.framebuffer[line][pixel] = (125, 125, 125);
        // }

        self.scanline_cycle += 1;
        // println!("scanline cycle is increased");
        if self.scanline_cycle >= HORIZONTAL_CYCLES {

            // TODO; Trigger WSYNc perhaps releasing CPU halt.
            self.cpu_halt = false;

            // println!("scanline is increased");
            self.scanline += 1;

            if self.scanline >= 3 + 37 && self.scanline < 3 + 37 + 192 {
                let line = self.scanline - (3 + 37);
                self.framebuffer[line as usize] = create_scanline(self);
            }

            if self.scanline >= VERTICAL_LINES {

                // clear out framebuffer  for debugging purpose
                self.framebuffer = [[(0, 0, 0); 160]; 192];

                self.scanline = 0;
            }

            self.scanline_cycle = 0;
        }
    }

    pub fn update_accurate(&mut self, elapsed_time: Duration) {

        self.elapsed_time += elapsed_time;

        while self.elapsed_time >= CYCLE_DURATION {
            self.elapsed_time -= CYCLE_DURATION;
            self.remaining_cycles += 1;
        }

        while self.remaining_cycles > 0 {
            if !self.cpu_halt {

                let mut elapsed_cycles = self.execute_instruction();
                self.remaining_cycles -= elapsed_cycles as isize;

                while elapsed_cycles > 0 {
                    self.execute_cycle();
                    elapsed_cycles -= 1;
                }

                self.timer_block = false;
            }
            else {
                while self.remaining_cycles > 0 {
                    self.execute_cycle();
                    self.remaining_cycles -= 1;

                    if !self.cpu_halt {
                        break
                    }
                }
            }
        }

    }

    /// Advance the simulation in time.
    ///
    /// This function must be called to advance the simulation in time. It's
    /// called with the elapsed time which should be as small as possible to
    /// avoid any 'time warp' effect.
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

        // Update our own elapsed time tracker.
        self.elapsed_time += elapsed_time;

        // A division with remainder could have been used but it's not provided
        // by the standard library, and it would likely result in poorer
        // performance anyway as modern machines run significantly faster than
        // the Atari 2600  (and thus the elapsed time is very small).
        while self.elapsed_time >= CYCLE_DURATION {
            self.elapsed_time -= CYCLE_DURATION;
            self.remaining_cycles += 1;
        }

        // It's inconvenient to compute how many cycles the next instruction will
        // take, but at the same time, we can't be ahead of the simulation.
        // However, we know it will never exceeds 7 cycles, so we'll do the
        // simulation 10 cycles at a time.
        //
        // Note that in the following loop, it doesn't mean we consume 10
        // cycles.
        while self.remaining_cycles >= 10 {

            if !self.cpu_halt {
                // When the CPU is not halted by the TIA, we simply execute a
                // CPU instruction. If the TIA is halting the CPU after the
                // execution of the instruction, we let the next iteration
                // process the remaining cycles.

                // Execute the next instruction (and update the iterator).
                let mut elapsed_cycles = self.execute_instruction();
                self.remaining_cycles -= elapsed_cycles as isize;

                // For each cycle that the instruction took, we execute 3 TIA
                // cycles.
                while elapsed_cycles > 0 {
                    self.execute_cycle();
                    elapsed_cycles -= 1;
                }

                self.timer_block = false;
            }
            else {
                // When the CPU is halted, we run only TIA cycles until the CPU
                // is released. As soon as it's release, we let the next
                // iteration continue the job (as it will immediately start
                // resume executing instructions).

                // For each remaining cycles to simulate, execute 3 TIA cycles.
                while self.remaining_cycles > 0 {
                    self.execute_cycle();
                    self.remaining_cycles -= 1;

                    // If the CPU is release, we stop here and let the next
                    // iteration execute the next instruction.
                    if !self.cpu_halt {
                        break
                    }
                }
            }
        }

        // If remaining cycles was less than 0, we'd be ahead of the simulation
        // and this is a logical error.
        assert!(self.remaining_cycles >= 0);
    }

    fn wait_for_leading_edge_of_horizontal_blank(&mut self) {
        // TODO; To be implemented.
        self.cpu_halt = true;
    }

    fn reset_horizontal_sync_counter(&mut self) {
        // TODO; To be implemented.
        // panic!("not implemented yet");

// 10h - RESP0 <strobe> - Reset player 0
// 11h - RESP1 <strobe> - Reset player 1
// 12h - RESM0 <strobe> - Reset missile 0
// 13h - RESM1 <strobe> - Reset missile 1
// 14h - RESBL <strobe> - Reset ball
// Writing any value to these addresses sets the associated objects horizontal
// position equal to the current position of the cathode ray beam, if the write
// takes place anywhere within horizontal blanking then the position is set to
// the left edge of the screen (plus a few pixels towards right: 3 pixels for P0/P1, and only 2 pixels for M0/M1/BL).
// Note: Because of opcode execution times, it is usually necessary to adjust
//the resulting position to the desired value by subsequently using the Horizontal Motion function.
    }

    fn reset_position(&mut self, position: &mut u32, is_player: bool) {
        if self.is_horizontal_blank() {
            // If the strobe register is triggered during horizontal blanking,
            // the position will become at the very left of the screen edge plus
            // 3 pixels for players, and 2 pixels for missiles and the ball.
            *position = if is_player { 3 } else { 2 };
        }
        else {
            *position = self.beam_position().1 as u32;
        }
    }

    fn reset_player_0(&mut self) {
        // self.reset_position(&mut self.players_position[0], true);
    }

    fn reset_player_1(&mut self) {
        // self.reset_position(&mut self.players_position[1], true);
    }

    fn reset_missile_0(&mut self) {
        // self.reset_position(&mut self.missiles_position[0], false);
    }

    fn reset_missile_1(&mut self) {
        // self.reset_position(&mut self.missiles_position[1], false);
    }

    fn reset_ball(&mut self) {
        // self.reset_position(&mut self.ball_position, false);
    }

    fn apply_horizontal_motion(&mut self) {
        // TODO; To be implemented.
        // panic!("not implemented yet");
    }

    fn clear_horizontal_motion_registers(&mut self) {
        // TODO; To be implemented.
        // panic!("not implemented yet");
    }

    fn clear_collision_latches(&mut self) {
        // Reset all collision-related bits to 0.
        *self.memory_mut(CXM0P)  = 0x0000_0000;
        *self.memory_mut(CXM1P)  = 0x0000_0000;
        *self.memory_mut(CXP0FB) = 0x0000_0000;
        *self.memory_mut(CXP1FB) = 0x0000_0000;
        *self.memory_mut(CXM0FB) = 0x0000_0000;
        *self.memory_mut(CXM1FB) = 0x0000_0000;
        *self.memory_mut(CXBLPF) = 0x0000_0000;
        *self.memory_mut(CXPPMM) = 0x0000_0000;
    }

    #[allow(mutable_transmutes)]
    pub(crate) fn memory<'a>(&self, mut index: u16) -> &'a u8 {
        // Cannot address more than 8192 bytes because bit 13, 14 and 15 are
        // ignored on the MOS 6507 (bus lines aren't attached).
        index &= 0b0001_1111_1111_1111;

        let reference = match index {
            0x_00..=0x_3D => &self.tia[index as usize],
            0x_80..=0x_FF => &self.ram[(index - 0x_80) as usize],

            // The PIA has 10 relevant memory locations but all timer-related
            // locations are mapped to local values. Last 4 aren't holding any
            // values and thus are mapped to dummy.
            0x_0280..=0x_0283 => &self.pia[(index - 0x_0280) as usize],
            0x_0284 => &self.timer_value,
            0x_0285 => {
                // Note: Technically, callers of this method usually have a
                // mutable reference of the console, and the signature of this
                // method should be changed to use `&mut self`. That said, it's
                // nicer this way for several reasons.

                unsafe {
                    // Whenever the INSTAT register is read, its 6th bit is reset.
                    let mut_self = std::mem::transmute::<&Console, &mut Console>(self);
                    mut_self.timer_status &= 0b1011_1111;
                }

                &self.timer_status
            },
            0x_0294..=0x_0297 => &self.dummy[index as usize],

            // This portion of the memory is mapped to the ROM on the cartridge
            // but it's varying from cartridge to cartridge.
            0x_1000..=0x_1FFF => &self.cartridge.memory[(index - 0x_1000) as usize],

            // Adressing an irrelevant memory location, just returning 0; it's
            // legal and it doesn't matter.
            //
            // TODO; Perhaps log this message, and also it could be a mapped
            // memory which is not supported yet by this emulator.
            _ => &self.dummy[index as usize]
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

            // The PIA has 10 relevant memory locations but all timer-related
            // locations are mapped to local values. Last 4 aren't holding any
            // values and thus are mapped to dummy.
            0x_0280..=0x_0283 => &mut self.pia[(index - 0x_0280) as usize],
            0x_0284 => {
                // I'm not sure if it's legal to write to this register
                // directly. Usually it's done via one of TIM1T, TIM8T, TIM64T
                // or T1024T registers. What would the side effect be ?
                println!("fishy ROM warning; is it legal to write to INTIM register ?");

                &mut self.timer_value
            },
            0x_0285 => {
                // Whenever the INSTAT register is read, its 6th bit is reset.
                self.timer_status &= 0b1011_1111;

                &mut self.timer_status
            },
            0x_0294..=0x_0297 => {
                // Adjust the timer interval accordingly.
                self.timer_interval = match index {
                    0x_0294 => 1,
                    0x_0295 => 8,
                    0x_0296 => 64,
                    0x_0297 => 1024,
                    _ => panic!("foo")
                };

                self.timer_block = true;

                // Whenever register TIM1T, TIM8T, TIM64T and T1024T are
                // written, it resets the 7th bit of INSTAT register.
                *self.memory_mut(INSTAT) &= 0b0111_1111;

                self.timer_elapsed_clocks = 1;

                // When those registers are written, it's actually updating the
                // value of the INTIM register (which is mapped to our local
                // value).
                &mut self.timer_value
            },

            // This portion of the memory is mapped to the ROM on the cartridge
            // but it's varying from cartridge to cartridge.
            0x_F000..=0x_FFFF => &mut self.cartridge.memory[(index - 0x_F000) as usize],
            // 0x_1000..=0x_1FFF => &mut self.cartridge.memory[(index - 0x_1000) as usize],

            // Adressing an irrelevant memory location, just returning 0; it's
            // legal and it doesn't matter.
            //
            // TODO; Perhaps log this message, and also it could be a mapped
            // memory which is not supported yet by this emulator.
            _ => &mut self.dummy[index as usize]
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
            0x_0A | 0x_06 | 0x_16 | 0x_0E | 0x_1E => asl_instruction(self, opcode),
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
            0x_E0 => cpx_instruction(self, opcode),
            0x_C0 | 0x_C4 | 0x_CC => cpy_instruction(self, opcode),
            0x_C6 | 0x_D6 | 0x_CE | 0x_DE => dec_instruction(self, opcode),
            0x_CA => dex_instruction(self, opcode),
            0x_88 => dey_instruction(self, opcode),
            0x_49 | 0x_45 | 0x_55 | 0x_4D | 0x_5D | 0x_59 | 0x_41 | 0x_51 => eor_instruction(self, opcode),
            0x_E6 | 0x_F6 | 0x_EE | 0x_FE => inc_instruction(self, opcode),
            0x_E8 => inx_instruction(self, opcode),
            0x_C8 => iny_instruction(self, opcode),
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
            0x_86 | 0x_96 | 0x_8E => stx_instruction(self, opcode),
            0x_84 | 0x_94 | 0x_8C => sty_instruction(self, opcode),
            0x_AA => tax_instruction(self, opcode),
            0x_A8 => tay_instruction(self, opcode),
            0x_BA => tsx_instruction(self, opcode),
            0x_8A => txa_instruction(self, opcode),
            0x_9A => txs_instruction(self, opcode),
            0x_98 => tya_instruction(self, opcode),
            _ => {
                println!("unknown instruction");
                0
                // panic!("unknown instruction")
            }
        };

        // Increase instructions count (for debugging and analysis).
        self.instructions_count += 1;

        cycles
    }

    // /// Brief description.
    // ///
    // /// Long description.
    // ///
    // pub(crate) fn set_input(index: usize, value: bool) {
    //     // 38      INPT0   1.......  read pot port
    //     // 39      INPT1   1.......  read pot port
    //     // 3A      INPT2   1.......  read pot port
    //     // 3B      INPT3   1.......  read pot port
    //     // 3C      INPT4   1.......  read input
    //     // 3D      INPT5   1.......  read input

    //     let memory_index = match index {
    //         0 => 0x_38,
    //         1 => 0x_39,
    //         2 => 0x_3A,
    //         3 => 0x_3B,
    //         4 => 0x_3C,
    //         5 => 0x_3D
    //     };

    //     // The other bits are unused. Don't be afraid to ovewrite.
    //     self.memory[memory_index] = if value { 0b1000_0000 } else { 0b0000_0000 };
    // }

    // /// Execute the next instruction.
    // ///
    // /// Long description to be written.
    // ///
    // pub(crate) fn set_switch_a(&mut self, pin: usize, value: bool) {

    //     assert!(pin < 8, "pin can't be higher than 7");

    //     let operand: u8 = 1 << pin;
    //     let new_value = self.memory(0x_0280) | operand;

    //     *self.memory_mut(0x_0280) = new_value;
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_foo() {

        // Test test01.bin

        let cartridge = Cartridge::from_file("/home/intjelic/Workspace/atari-2600/kernel_01.bin").unwrap();
        let mut console = Console::new(cartridge);

        for _ in 0..10000 {
            // println!("test");
            console.update(CYCLE_DURATION);
        }


    }

    #[test]
    fn test_subroutine() {
        // A quick test to make sure subroutines work.
        let mut console = Console::new(Cartridge::new(vec![]));

        // TODO; To be implemented.

        // setup_instruction(&mut console, vec![0x_6C, 0x_42, 0x_31, 0x_C8]);
        // *console.memory_mut(0x_3142) = 0x_E8;
        // *console.memory_mut(0x_3142 + 1) = 0x_60;

        // let cycles = execute_instruction(&mut console, jrs_instruction);
        // let cycles = execute_instruction(&mut console, inx_instruction);
        // let cycles = execute_instruction(&mut console, rts_instruction);
        // let cycles = execute_instruction(&mut console, iny_instruction);

    }

    #[test]
    fn test_timer() {
        // Test timer-related functionalities (performed by the PIA).

        // Create a ROM to put the console into different states and check if
        // the states are correct.
        let cartridge = Cartridge::new(vec![
            0x_A9, 0x_05,        // Load accumulator with value 5
            0x_8D, 0x_95, 0x_02, // Write to register TIM8T with the accumulator value
            // Do 2 times 8 'do nothing' cycles.
            0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA,
            0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA,
            // During this 8 cycles, read the INSTAT register (don't be confused with EA and AE)
            0x_EA, 0x_EA, 0x_AE, 0x_85, 0x_02, 0x_EA, 0x_EA, 0x_EA,
            // Do 2 times 8 'do nothing' cycles.
            0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA,
            // Do 2 times 'do nothing' cycles to finsih the testing.
            0x_EA, 0x_EA,
            0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA,
            0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA,
            0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA,
            0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA, 0x_EA,
        ]);

        // Create the console and advance the simulation slightly forward to
        // avoid being on the cycle edges.
        let mut console = Console::new(cartridge);
        console.update_accurate(CYCLE_DURATION / 10); // slightly advance the simulation

        assert_eq!(console.timer_value, 0);
        assert_eq!(console.timer_status & 0b_0100_0000 != 0, false);
        assert_eq!(console.timer_status & 0b_1000_0000 != 0, false);
        assert_eq!(console.timer_interval, 1);

        // Advance the simulation by 2 cycles. At this time, the accumulator is
        // loaded with value 5.
        console.update_accurate(CYCLE_DURATION * 2);
        assert_eq!(console.accumulator, 5);

        // Advance the simulation by 4 cycles. At this time, the register TIM8T
        // has been written with the value of the accumulator (which is 5). The
        // register INTIM is updated and the register INSTAT 7th bit is reset.
        console.timer_status |= 0b_1000_000;
        console.update_accurate(CYCLE_DURATION * 4);
        assert_eq!(console.timer_value, 5);
        assert_eq!(console.timer_status & 0b_1000_0000 != 0, false);

        // The timer is immediately decremented after the first cycle.
        console.update_accurate(CYCLE_DURATION);
        assert_eq!(console.timer_value, 4);

        // Then after that, it's taking 8 cycles for the next decrement.
        console.update_accurate(CYCLE_DURATION * 8);
        assert_eq!(console.timer_value, 3);

        // During the next 8 cycles, the INSTAT register is read which should
        // reset the 6th bit of INSTAT register.
        console.update_accurate(CYCLE_DURATION * 2);

        console.timer_status |= 0b_0100_000;
        console.update_accurate(CYCLE_DURATION * 3);
        assert_eq!(console.timer_status & 0b_0100_0000 != 0, false);

        console.update_accurate(CYCLE_DURATION * 3);
        assert_eq!(console.timer_value, 2);

        // Run another 2 times more 8 cycles for the timer value to finally
        // reach 0.
        console.update_accurate(CYCLE_DURATION * 16);
        assert_eq!(console.timer_value, 0);

        console.update_accurate(CYCLE_DURATION);
        console.update_accurate(CYCLE_DURATION);
        console.update_accurate(CYCLE_DURATION);
        console.update_accurate(CYCLE_DURATION);
        console.update_accurate(CYCLE_DURATION);
        console.update_accurate(CYCLE_DURATION);
        // console.update_accurate(CYCLE_DURATION);

        // Then it's high speed decrement, timer values underflows and become
        // 255.
        console.timer_status &= 0b_0011_1111; // reset 6th and 7th bit
        console.update_accurate(CYCLE_DURATION);
        assert_eq!(console.timer_value, 0x_FF);
        assert_eq!(console.timer_status & 0b_0100_0000 != 0, true);
        assert_eq!(console.timer_status & 0b_1000_0000 != 0, true);

        console.update_accurate(CYCLE_DURATION);
        assert_eq!(console.timer_value, 0x_FE);

        console.update_accurate(CYCLE_DURATION);
        assert_eq!(console.timer_value, 0x_FD);

        // console.update_accurate(CYCLE_DURATION);
        // assert_eq!(console.timer_value, 255);

        // TODO; This unit test is not completed.
    }
}