/// This module loads the second stage image into memory.

use core::{intrinsics::transmute, marker::PhantomData};

use i386::bios::disk::{DAPError, size_to_lba};
use i386::fs::{FSError, FileSystem, nofs::real::NoFSReal};
use shared::layout::{STAGE1_SIZE, STAGE2_SIZE, STAGE3_SIZE, STAGE3_START};

/// The address of the second stage image.


/// Just a tag indicating where is the second stage. The .stage_2 section
/// is a dummy section whose size is 0.
#[link_section = ".stage_3"]
pub static STAGE3_PTR: PhantomData<()> = PhantomData;

pub const STAGE_DISK: u8 = 0x80;

#[inline]
pub fn load_stage3() -> Result<(), FSError<DAPError>> {
    let fs = NoFSReal::new(STAGE_DISK)?;

    let stage_3 = unsafe { 
        transmute::<usize, &mut [u8; STAGE3_SIZE]>(STAGE3_START) 
    };
    let start_lba = size_to_lba(STAGE1_SIZE + STAGE2_SIZE);

    fs.read(start_lba, stage_3)?;
    Ok(())
}

