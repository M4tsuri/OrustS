use crate::{
    fs::{FSError, FileSystem},
    driver::disk::dap::*
};

use super::NoFSIdent;

/// The nofs driver under real mode, the hard access is made with BIOS interrupts
pub struct NoFSReal {
    drive: u8
}

impl NoFSReal {
    pub fn new(drive: u8) -> Result<Self, FSError<DAPError>> {
        reset_disk(drive)?;
        Ok(Self {
            drive
        })
    }
}

impl FileSystem<NoFSIdent, DAPError> for NoFSReal {
    fn alloc(&mut self, _size: usize) -> Result<NoFSIdent, FSError<DAPError>> {
        Err(FSError::NotImplemented)
    }

    fn extend(&mut self, _orig: NoFSIdent, _new_size: usize) -> Result<NoFSIdent, FSError<DAPError>> {
        Err(FSError::NotImplemented)
    }

    fn delete(&mut self, _id: NoFSIdent) -> Result<(), FSError<DAPError>> {
        Err(FSError::NotImplemented)
    }

    fn write(&mut self, _id: NoFSIdent, _src: &[u8]) -> Result<usize, FSError<DAPError>> {
        Err(FSError::NotImplemented)
    }

    fn read(&self, id: NoFSIdent, dest: &mut [u8]) -> Result<usize, FSError<DAPError>> {
        read_disk((self.drive, id as u64), dest)?;
        Ok(dest.len())
    }
}