// Copyright (c) 2020 - Jonathan De Wachter
//
// This source file is part of Atari 2600 Emulator which is released under the
// MIT license. Please refer to the LICENSE file that can be found at the root
// of the project directory.
//
// Written by Jonathan De Wachter <dewachter.jonathan@gmail.com>, November 2020

// Some constants may never be used in the code but it's nice to keep an
// exhaustive list of the constants.
#![allow(dead_code)]

// Each memory location has official label; find some constants here to make
// the code more readable. The list was taken from there:
// https://problemkaputt.de/2k6specs.htm#controllersjoysticks
pub(crate) const VSYNC  : u16 = 0x_0000; // ......1.  vertical sync set-clear
pub(crate) const VBLANK : u16 = 0x_0001; // 11....1.  vertical blank set-clear
pub(crate) const WSYNC  : u16 = 0x_0002; // <strobe>  wait for leading edge of horizontal blank
pub(crate) const RSYNC  : u16 = 0x_0003; // <strobe>  reset horizontal sync counter
pub(crate) const NUSIZ0 : u16 = 0x_0004; // ..111111  number-size player-missile 0
pub(crate) const NUSIZ1 : u16 = 0x_0005; // ..111111  number-size player-missile 1
pub(crate) const COLUP0 : u16 = 0x_0006; // 1111111.  color-lum player 0 and missile 0
pub(crate) const COLUP1 : u16 = 0x_0007; // 1111111.  color-lum player 1 and missile 1
pub(crate) const COLUPF : u16 = 0x_0008; // 1111111.  color-lum playfield and ball
pub(crate) const COLUBK : u16 = 0x_0009; // 1111111.  color-lum background
pub(crate) const CTRLPF : u16 = 0x_000A; // ..11.111  control playfield ball size & collisions
pub(crate) const REFP0  : u16 = 0x_000B; // ....1...  reflect player 0
pub(crate) const REFP1  : u16 = 0x_000C; // ....1...  reflect player 1
pub(crate) const PF0    : u16 = 0x_000D; // 1111....  playfield register byte 0
pub(crate) const PF1    : u16 = 0x_000E; // 11111111  playfield register byte 1
pub(crate) const PF2    : u16 = 0x_000F; // 11111111  playfield register byte 2
pub(crate) const RESP0  : u16 = 0x_0010; // <strobe>  reset player 0
pub(crate) const RESP1  : u16 = 0x_0011; // <strobe>  reset player 1
pub(crate) const RESM0  : u16 = 0x_0012; // <strobe>  reset missile 0
pub(crate) const RESM1  : u16 = 0x_0013; // <strobe>  reset missile 1
pub(crate) const RESBL  : u16 = 0x_0014; // <strobe>  reset ball
pub(crate) const AUDC0  : u16 = 0x_0015; // ....1111  audio control 0
pub(crate) const AUDC1  : u16 = 0x_0016; // ....1111  audio control 1
pub(crate) const AUDF0  : u16 = 0x_0017; // ...11111  audio frequency 0
pub(crate) const AUDF1  : u16 = 0x_0018; // ...11111  audio frequency 1
pub(crate) const AUDV0  : u16 = 0x_0019; // ....1111  audio volume 0
pub(crate) const AUDV1  : u16 = 0x_001A; // ....1111  audio volume 1
pub(crate) const GRP0   : u16 = 0x_001B; // 11111111  graphics player 0
pub(crate) const GRP1   : u16 = 0x_001C; // 11111111  graphics player 1
pub(crate) const ENAM0  : u16 = 0x_001D; // ......1.  graphics (enable) missile 0
pub(crate) const ENAM1  : u16 = 0x_001E; // ......1.  graphics (enable) missile 1
pub(crate) const ENABL  : u16 = 0x_001F; // ......1.  graphics (enable) ball
pub(crate) const HMP0   : u16 = 0x_0020; // 1111....  horizontal motion player 0
pub(crate) const HMP1   : u16 = 0x_0021; // 1111....  horizontal motion player 1
pub(crate) const HMM0   : u16 = 0x_0022; // 1111....  horizontal motion missile 0
pub(crate) const HMM1   : u16 = 0x_0023; // 1111....  horizontal motion missile 1
pub(crate) const HMBL   : u16 = 0x_0024; // 1111....  horizontal motion ball
pub(crate) const VDELP0 : u16 = 0x_0025; // .......1  vertical delay player 0
pub(crate) const VDELP1 : u16 = 0x_0026; // .......1  vertical delay player 1
pub(crate) const VDELBL : u16 = 0x_0027; // .......1  vertical delay ball
pub(crate) const RESMP0 : u16 = 0x_0028; // ......1.  reset missile 0 to player 0
pub(crate) const RESMP1 : u16 = 0x_0029; // ......1.  reset missile 1 to player 1
pub(crate) const HMOVE  : u16 = 0x_002A; // <strobe>  apply horizontal motion
pub(crate) const HMCLR  : u16 = 0x_002B; // <strobe>  clear horizontal motion registers
pub(crate) const CXCLR  : u16 = 0x_002C; // <strobe>  clear collision latches

pub(crate) const CXM0P  : u16 = 0x_0030; // 11......  read collision M0-P1, M0-P0 (Bit 7,6)
pub(crate) const CXM1P  : u16 = 0x_0031; // 11......  read collision M1-P0, M1-P1
pub(crate) const CXP0FB : u16 = 0x_0032; // 11......  read collision P0-PF, P0-BL
pub(crate) const CXP1FB : u16 = 0x_0033; // 11......  read collision P1-PF, P1-BL
pub(crate) const CXM0FB : u16 = 0x_0034; // 11......  read collision M0-PF, M0-BL
pub(crate) const CXM1FB : u16 = 0x_0035; // 11......  read collision M1-PF, M1-BL
pub(crate) const CXBLPF : u16 = 0x_0036; // 1.......  read collision BL-PF, unused
pub(crate) const CXPPMM : u16 = 0x_0037; // 11......  read collision P0-P1, M0-M1
pub(crate) const INPT0  : u16 = 0x_0038; // 1.......  read pot port
pub(crate) const INPT1  : u16 = 0x_0039; // 1.......  read pot port
pub(crate) const INPT2  : u16 = 0x_003A; // 1.......  read pot port
pub(crate) const INPT3  : u16 = 0x_003B; // 1.......  read pot port
pub(crate) const INPT4  : u16 = 0x_003C; // 1.......  read input
pub(crate) const INPT5  : u16 = 0x_003D; // 1.......  read input

pub(crate) const SWCHA  : u16 = 0x_0280; //  11111111  Port A; input or output  (read or write)
pub(crate) const SWACNT : u16 = 0x_0281; //  11111111  Port A DDR, 0= input, 1=output
pub(crate) const SWCHB  : u16 = 0x_0282; //  11111111  Port B; console switches (read only)
pub(crate) const SWBCNT : u16 = 0x_0283; //  11111111  Port B DDR (hardwired as input)
pub(crate) const INTIM  : u16 = 0x_0284; //  11111111  Timer output (read only)
pub(crate) const INSTAT : u16 = 0x_0285; //  11......  Timer Status (read only, undocumented)
pub(crate) const TIM1T  : u16 = 0x_0294; //  11111111  set 1 clock interval (838 nsec/interval)
pub(crate) const TIM8T  : u16 = 0x_0295; //  11111111  set 8 clock interval (6.7 usec/interval)
pub(crate) const TIM64T : u16 = 0x_0296; //  11111111  set 64 clock interval (53.6 usec/interval)
pub(crate) const T1024T : u16 = 0x_0297; //  11111111  set 1024 clock interval (858.2 usec/interval)
