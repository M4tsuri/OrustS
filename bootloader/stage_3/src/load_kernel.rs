use core::intrinsics::transmute;

use alloc::string::String;
/// The module provides function for loading kernel from disk into memory.
/// Since we need to load kernel to 1MB, which exceeds the real mode addressing
/// limit, we use ATA command to do this work.

use i386::{bios::disk::SECTOR_ALIGN, hardware::ata::ATADriver};
use shared::layout::*;

pub fn load_kernel() -> Result<(), String> {
    let kernel_lba = (STAGE1_SIZE + STAGE2_SIZE + STAGE3_SIZE) >> SECTOR_ALIGN;
    
    let res = ATADriver::PRIMARY.pio_identify()
        .map_err(|_| "Disk Error when identifying.\n")?;
    let lba48_secs = 
        res[200] as u64 | 
        (res[201] as u64) << 8 |
        (res[202] as u64) << 16 |
        (res[203] as u64) << 24 |
        (res[204] as u64) << 32 |
        (res[205] as u64) << 40;
    
    let kernel_buf = unsafe { 
        transmute::<usize, &mut [u8; KERNEL_SIZE]>(KERNEL_START)
    };
    ATADriver::PRIMARY.pio_read_sectors(
        kernel_lba as u64, 
        kernel_buf, 
        lba48_secs - kernel_lba as u64
    ).map_err(|x| x.into())
}
