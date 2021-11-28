/// This segment is overlapped with the code segment.
/// It should only be used for protect-real mode switching.
/// The base address and limit of this segment provides us with appreciated values 
/// for mode switching and real mode addressing.

pub const I386_MAX_ADDR: usize = 0xffffffff;
pub const NORMAL_START: usize = 0;
/// NORMAL size must be 0x10000.
pub const NORMAL_SIZE: usize = 0x10000;
pub const NORMAL_END: usize = NORMAL_START + NORMAL_SIZE;


pub const CODE_START: usize = 0;
pub const CODE_END: usize = I386_MAX_ADDR;

pub const DATA_START: usize = 0;
pub const DATA_END: usize = I386_MAX_ADDR;

pub const STACK_START: usize = 327680;
pub const STACK_END: usize = 393216;

pub const VIDEO_START: usize = 753664;
/// 80x25 16 bit text mode 
pub const VIDEO_SIZE: usize = 80 * 25 * 2;
pub const VIDEO_END: usize = VIDEO_START + VIDEO_SIZE;

pub const STAGE1_START: usize = 31744;

pub const STAGE1_END: usize = 32256;
pub const STAGE2_START: usize = 32256;
pub const GDT_START: usize = 65280;
pub const MEMINFO_START: usize = 65024;
pub const MEMINFO_SIZE: usize = GDT_START - MEMINFO_START;

pub const GDT_END: usize = 65536;
pub const GDT_SIZE: usize = GDT_END - GDT_START;
pub const STAGE2_END: usize = 65536;
pub const STAGE3_START: usize = 65536;

pub const STAGE3_END: usize = 327680;

pub const KERNEL_START: usize = 1048576;
pub const KERNEL_END: usize = 2097152;

pub const REAL_MODE_MAX_ADDRESS: usize = 0x100000;

pub const CODE_SIZE: usize = CODE_END - CODE_START;
pub const DATA_SIZE: usize = DATA_END - DATA_START;
pub const STACK_SIZE: usize = STACK_END - STACK_START;

pub const STAGE1_SIZE: usize = STAGE1_END - STAGE1_START;
pub const STAGE2_SIZE: usize = STAGE2_END - STAGE2_START;
pub const STAGE3_SIZE: usize = STAGE3_END - STAGE3_START;

pub const KERNEL_SIZE: usize = KERNEL_END - KERNEL_START;
