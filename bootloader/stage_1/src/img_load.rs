/// This module loads the second stage image into memory.

use core::{intrinsics::transmute, marker::PhantomData};
use i386::bios::disk::{SECTOR_ALIGN, read_disk, reset_disk};
use shared::layout::{STAGE1_SIZE, STAGE2_SIZE, STAGE2_START};

/// The address of the second stage image.


/// Just a tag indicating where is the second stage. The .stage_2 section
/// is a dummy section whose size is 0.
#[link_section = ".stage_2"]
pub static STAGE2_PTR: PhantomData<()> = PhantomData;

/// The disk index of the first hard disk.
pub const STAGE_DISK: u8 = 0x80;

#[inline]
pub fn load_stage2() -> Result<(), &'static str> {
    reset_disk(STAGE_DISK)?;
    let stage_2 = unsafe { transmute::<usize, &mut [u8; STAGE2_SIZE]>(STAGE2_START) };
    read_disk((STAGE_DISK, (STAGE1_SIZE >> SECTOR_ALIGN) as u64), stage_2)?;
    Ok(())
}
