/// This module loads the second stage image into memory.

use core::{intrinsics::transmute, marker::PhantomData};

use i386::disk::{SECTOR_ALIGN, read_disk, reset_disk};
use shared::layout::{STAGE1_SIZE, STAGE2_SIZE, STAGE3_SIZE, STAGE3_START};

/// The address of the second stage image.


/// Just a tag indicating where is the second stage. The .stage_2 section
/// is a dummy section whose size is 0.
#[link_section = ".stage_3"]
pub static STAGE3_PTR: PhantomData<()> = PhantomData;
pub const STAGE_DISK: u8 = 0x80;

#[inline]
pub fn load_stage3() -> Result<(), &'static str> {
    reset_disk(STAGE_DISK)?;
    let stage_3 = unsafe { transmute::<usize, &mut [u8; STAGE3_SIZE]>(STAGE3_START) };
    let start_lba = ((STAGE1_SIZE + STAGE2_SIZE) >> SECTOR_ALIGN) as u64;
    read_disk((STAGE_DISK, start_lba), stage_3)?;
    Ok(())
}
