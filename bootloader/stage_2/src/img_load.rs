/// This module loads the second stage image into memory.

use core::{intrinsics::transmute, marker::PhantomData};

use i386::bios::disk::{SECTOR_ALIGN, read_disk, reset_disk};
use shared::layout::{KERNEL_SIZE, KERNEL_START, STAGE1_SIZE, STAGE2_SIZE, STAGE3_SIZE, STAGE3_START};

/// The address of the second stage image.


/// Just a tag indicating where is the second stage. The .stage_2 section
/// is a dummy section whose size is 0.
#[link_section = ".stage_3"]
pub static STAGE3_PTR: PhantomData<()> = PhantomData;

pub const STAGE_DISK: u8 = 0x80;

#[inline]
pub fn load_stage3() -> Result<(), &'static str> {
    reset_disk(STAGE_DISK)?;
    let stage_3 = unsafe { 
        transmute::<usize, &mut [u8; STAGE3_SIZE]>(STAGE3_START) 
    };
    let start_lba = ((STAGE1_SIZE + STAGE2_SIZE) >> SECTOR_ALIGN) as u64;
    read_disk((STAGE_DISK, start_lba), stage_3)?;
    Ok(())
}

/// By convinence, we put kernel after bootloader on disk.
/// Currently the size of kernel must be less than 1MB.
pub fn load_kernel() -> Result<(), &'static str> {
    let start_lba = ((STAGE1_SIZE + STAGE2_SIZE + STAGE3_SIZE) >> SECTOR_ALIGN) as u64;
    reset_disk(STAGE_DISK)?;
    let kernel = unsafe {
        transmute::<usize, &mut [u8; KERNEL_SIZE]>(KERNEL_START)
    };
    read_disk((STAGE_DISK, start_lba), kernel)?;
    Ok(())
}
