#![no_std]

pub const CODE_START: u32 = 0;
pub const CODE_END: u32 = 0x80000;

pub const DATA_START: u32 = 0x80000;
pub const DATA_END: u32 = 0x9c000;

pub const VIDEO_START: u32 = 0xb8000;

pub const STACK_START: u32 = 0x9c000;
pub const STACK_END: u32 = 0xb8000 - 0x10;

pub const STAGE1_START: u32 = CODE_START + 0x7c00;
pub const STAGE1_SIZE: u32 = 512;
pub const STAGE1_END: u32 = STAGE1_START + STAGE1_SIZE;

pub const STAGE2_START: u32 = STAGE1_END;
pub const STAGE2_END: u32 = 0x10000;
pub const STAGE2_SIZE: u32 = STAGE2_END - STAGE2_START;


pub const STAGE3_START: u32 = STAGE2_END;
pub const STAGE3_END: u32 = DATA_END;
pub const STAGE3_SIZE: u32 = STAGE3_END - STAGE3_START;
