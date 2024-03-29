//! The module provides function for loading kernel from disk into memory.
//! Since we need to load kernel to 1MB, which exceeds the real mode addressing
//! limit, we use ATA command to do this work.

use core::{
    intrinsics::transmute,
    marker::PhantomData
};
use alloc::string::String;
use i386::{
    utils::disk::size_to_lba,
    driver::disk::ata::ATAError,
    fs::{
        FSError, 
        FileSystem,
        nofs::protected::NoFSProtected
    },
};
use shared::layout::*;

#[link_section = ".kernel"]
pub static KERNEL_PTR: PhantomData<()> = PhantomData;

pub fn load_kernel(fs: &NoFSProtected) -> Result<(), String> {
    let kernel_lba = size_to_lba(STAGE1_SIZE + STAGE2_SIZE + STAGE3_SIZE);
    
    let kernel_buf = unsafe { 
        transmute::<usize, &mut [u8; KERNEL_SIZE]>(KERNEL_START)
    };

    fs.read(kernel_lba, kernel_buf)
        .map_err(|x| <FSError<ATAError> as Into<String>>::into(x))?;
    Ok(())
}
