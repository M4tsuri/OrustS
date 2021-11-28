use core::intrinsics::transmute;

/// The module provides function for loading kernel from disk into memory.
/// Since we need to load kernel to 1MB, which exceeds the real mode addressing
/// limit, we use ATA command to do this work.

use i386::{bios::disk::SECTOR_ALIGN, hardware::ata::ATADriver};
use shared::layout::*;

pub fn load_kernel() -> Result<(), &'static str> {
    let kernel_buf = unsafe { 
        transmute::<usize, &mut [u8; KERNEL_SIZE]>(KERNEL_START)
    };
    let kernel_lba = (STAGE1_SIZE + STAGE2_SIZE + STAGE3_SIZE) >> SECTOR_ALIGN;
    
    let res = ATADriver::PRIMARY.pio_identify()
        .map_err(|_| "Disk Error when identifying.\n")?;
    ATADriver::PRIMARY.pio_read_sectors(kernel_lba as u64, kernel_buf)
        .map_err(|_| "Disk Error when reading.\n")
}
