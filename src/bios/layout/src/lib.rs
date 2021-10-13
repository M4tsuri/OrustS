#![no_std]

/// This segment is overlapped with the code segment.
/// It should only be used for protect-real mode switching.
/// The base address and limit of this segment provides us with appreciated values 
/// for mode switching and real mode addressing.
pub const NORMAL_START: usize = 0;
/// NORMAL size must be 0x10000.
pub const NORMAL_SIZE: usize = 0x10000;
pub const NORMAL_END: usize = NORMAL_START + NORMAL_SIZE;


pub const CODE_START: usize = 0;
pub const CODE_END: usize = 491520;

pub const LDT_START: usize = 491520;
pub const LDT_END: usize = 495616;

pub const DATA_START: usize = 499712;
/// STAGE 3 ends here
pub const DATA_END: usize = 589824;

pub const STACK_START: usize = 589824;
pub const STACK_END: usize = 655360;

pub const VIDEO_START: usize = 753664;
/// 80x25 16 bit text mode 
pub const VIDEO_SIZE: usize = 80 * 25 * 2;
pub const VIDEO_END: usize = VIDEO_START + VIDEO_SIZE;
pub const CODE_SIZE: usize = CODE_END - CODE_START;
pub const DATA_SIZE: usize = DATA_END - DATA_START;
pub const STACK_SIZE: usize = STACK_END - STACK_START;
pub const LDT_SIZE: usize = LDT_END - LDT_START;

pub const STAGE1_START: usize = 31744;
pub const STAGE1_END: usize = 32256;
pub const STAGE2_START: usize = 32256;
pub const STAGE2_END: usize = 61440;
pub const STAGE3_START: usize = 61440;
pub const STAGE3_END: usize = 524288;
pub const REAL_MODE_MAX_ADDRESS: usize = 0x100000;
pub const STAGE1_SIZE: usize = STAGE1_END - STAGE1_START;
pub const STAGE2_SIZE: usize = STAGE2_END - STAGE2_START;
pub const STAGE3_SIZE: usize = STAGE3_END - STAGE3_START;