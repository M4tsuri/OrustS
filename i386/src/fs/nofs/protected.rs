extern crate alloc;
use alloc::string::String;

use crate::bios::disk::{is_sector_aligned, lba_to_size, size_to_lba};
use crate::fs::{FSError, FileSystem};
use crate::hardware::ata::{ATADriver, ATAError};

use super::NoFSIdent;

impl<T: Into<String>> Into<String> for FSError<T> {
    fn into(self) -> String {
        match self {
            FSError::DiskError(x) => x.into(),
            _ => "FS Error".into()
        }
    }
}

/// The nofs driver for protected mode, the hard disk access is made with ATA PIO
pub struct NoFSProtected {
    drive: ATADriver,
    sector_num: u64,
}

impl NoFSProtected {
    pub fn new(drive: ATADriver) -> Result<Self, FSError<ATAError>> {
        let res = drive.pio_identify()?;
        Ok(Self {
            drive,
            sector_num: res[200] as u64 | 
            (res[201] as u64) << 8 |
            (res[202] as u64) << 16 |
            (res[203] as u64) << 24 |
            (res[204] as u64) << 32 |
            (res[205] as u64) << 40
        }) 
    }
}

impl FileSystem<NoFSIdent, ATAError> for NoFSProtected {
    fn alloc(&mut self, _size: usize) -> Result<NoFSIdent, FSError<ATAError>> {
        Err(FSError::NotImplemented)
    }

    fn extend(&mut self, _orig: NoFSIdent, _new_size: usize) -> Result<NoFSIdent, FSError<ATAError>> {
        Err(FSError::NotImplemented)
    }

    fn delete(&mut self, _id: NoFSIdent) -> Result<(), FSError<ATAError>> {
        Err(FSError::NotImplemented)
    }

    fn write(&mut self, _id: NoFSIdent, _src: &[u8]) -> Result<usize, FSError<ATAError>> {
        Err(FSError::NotImplemented)
    }

    fn read(&self, lba: NoFSIdent, dest: &mut [u8]) -> Result<usize, FSError<ATAError>> {
        if !is_sector_aligned(dest.len()) {
            return Err(FSError::DiskError(ATAError::BufferNotAligned))
        }
        let mut sector_num = size_to_lba(dest.len());
        let remained_sector = self.sector_num - lba;

        if sector_num > remained_sector {
            sector_num = remained_sector;
        }

        self.drive.pio_read_sectors(
            lba as u64, 
            dest,
            sector_num
        )?;

        Ok((lba_to_size(sector_num)).try_into().map_err(|_| FSError::UnknownError)?)
    }
}

