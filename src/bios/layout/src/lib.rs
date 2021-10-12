#![no_std]

/// This segment is overlapped with the code segment.
/// It should only be used for protect-real mode switching.
/// The base address and limit of this segment provides us with appreciated values 
/// for mode switching and real mode addressing.
pub const NORMAL_START: u32 = 0;
/// NORMAL size must be 0x10000.
pub const NORMAL_SIZE: u32 = 0x10000;
pub const NORMAL_END: u32 = NORMAL_START + NORMAL_SIZE;


pub const CODE_START: u32 = 0;
pub const CODE_END: u32 = 491520;

pub const LDT_START: u32 = 491520;
pub const LDT_END: u32 = 495616;

pub const DATA_START: u32 = 499712;
/// STAGE 3 ends here
pub const DATA_END: u32 = 589824;

pub const STACK_START: u32 = 589824;
pub const STACK_END: u32 = 655360;

pub const VIDEO_START: u32 = 753664;
/// 80x25 16 bit text mode 
pub const VIDEO_SIZE: u32 = 80 * 25 * 2;
pub const VIDEO_END: u32 = VIDEO_START + VIDEO_SIZE;
pub const CODE_SIZE: u32 = CODE_END - CODE_START;
pub const DATA_SIZE: u32 = DATA_END - DATA_START;
pub const STACK_SIZE: u32 = STACK_END - STACK_START;
pub const LDT_SIZE: u32 = LDT_END - LDT_START;

pub const STAGE1_START: u32 = 31744;
pub const STAGE1_END: u32 = 32256;
pub const STAGE2_START: u32 = 32256;
pub const STAGE2_END: u32 = 65536;
pub const STAGE3_START: u32 = 65536;
pub const STAGE3_END: u32 = 524288;
pub const REAL_MODE_MAX_ADDRESS: u32 = 0x100000;
pub const STAGE1_SIZE: u32 = STAGE1_END - STAGE1_START;
pub const STAGE2_SIZE: u32 = STAGE2_END - STAGE2_START;
pub const STAGE3_SIZE: u32 = STAGE3_END - STAGE3_START;