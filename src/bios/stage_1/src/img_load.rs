/// This module loads the second stage image into memory.

use core::marker::PhantomData;
use i386::disk::{DAP, SECTOR_ALIGN};
use layout::{STAGE1_SIZE, STAGE2_SIZE, STAGE2_START};

/// The address of the second stage image.


/// Just a tag indicating where is the second stage. The .stage_2 section
/// is a dummy section whose size is 0.
#[link_section = ".stage_2"]
pub static STAGE2_PTR: PhantomData<()> = PhantomData;
pub const STAGE_DISK: u8 = 0x80;

#[inline]
pub fn load_stage2() -> Result<(), &'static str> {
    let dap = DAP::new(
        (STAGE_DISK, (STAGE1_SIZE >> SECTOR_ALIGN) as u64), 
        (STAGE2_START as u16, 0),
        STAGE2_SIZE);
    dap.reset()?;
    dap.read()?;
    Ok(())
}
