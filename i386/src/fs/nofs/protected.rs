extern crate alloc;
use alloc::string::String;

use crate::utils::disk::{is_sector_aligned, lba_to_size, size_to_lba};
use crate::fs::{FSError, FileSystem};
use crate::driver::disk::ata::pio::ATADiskInfo;
use crate::driver::disk::ata::{ATADriver, ATAError};
use crate::utils::u8x::CastUp;

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
    disk_info: ATADiskInfo
}

impl NoFSProtected {
    pub fn new(drive: ATADriver) -> Result<Self, FSError<ATAError>> {
        let disk_info = drive.pio_identify()?;
        Ok(Self {
            drive,
            disk_info
        })
    }

    pub fn get_disk_info(self) -> ATADiskInfo {
        self.disk_info
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
        let max_sector: u64 = self.disk_info.lba48_sec.cast_le();
        let remained_sector: u64 = max_sector - lba;

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

