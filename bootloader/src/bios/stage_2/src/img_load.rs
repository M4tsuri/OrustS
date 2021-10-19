/// This module loads the second stage image into memory.

use core::marker::PhantomData;

use i386::disk::{DAP, SECTOR_ALIGN};
use shared::layout::{STAGE1_SIZE, STAGE2_SIZE, STAGE3_SIZE, STAGE3_START};

/// The address of the second stage image.


/// Just a tag indicating where is the second stage. The .stage_2 section
/// is a dummy section whose size is 0.
#[link_section = ".stage_3"]
pub static STAGE3_PTR: PhantomData<()> = PhantomData;
pub const STAGE_DISK: u8 = 0x80;

#[inline]
pub fn load_stage3() -> Result<(), &'static str> {
    let dap = DAP::new(
        (STAGE_DISK, ((STAGE1_SIZE + STAGE2_SIZE) >> SECTOR_ALIGN) as u64), 
        (STAGE3_START as u16, ((STAGE3_START >> 16) as u16) << 12),
        STAGE3_SIZE);
    dap.reset()?;
    dap.read()?;
    Ok(())
}
