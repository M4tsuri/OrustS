/// This module loads the second stage image into memory.

use core::{intrinsics::transmute, marker::PhantomData};
use i386::utils::disk::size_to_lba;
use i386::fs::{FSError, FileSystem, nofs::real::NoFSReal};
use i386::driver::disk::dap::DAPError;
use shared::layout::{STAGE1_SIZE, STAGE2_SIZE, STAGE2_START};

/// The address of the second stage image.


/// Just a tag indicating where is the second stage. The .stage_2 section
/// is a dummy section whose size is 0.
#[link_section = ".stage_2"]
pub static STAGE2_PTR: PhantomData<()> = PhantomData;

/// The disk index of the first hard disk.
pub const STAGE_DISK: u8 = 0x80;

#[inline]
pub fn load_stage2() -> Result<(), FSError<DAPError>> {
    let fs = NoFSReal::new(STAGE_DISK)?;
    let lba = size_to_lba(STAGE1_SIZE);
    let stage_2 = unsafe { transmute::<usize, &mut [u8; STAGE2_SIZE]>(STAGE2_START) };

    fs.read(lba, stage_2)?;
    Ok(())
}
