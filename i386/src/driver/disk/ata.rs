//! Support for some useful ATA PIO commands
//! See https://wiki.osdev.org/ATA_PIO_Mode

pub mod pio;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::{format, string::String};
use crate::instrs::inb;


#[repr(u8)]
enum ATADCR {
    /// software reset, this command should be sent to Device Control Register
    SFTRST = 0b00000100,
    /// bus reset, this command should be sent to Device Control Register
    BUSRST = 0b00000000
}

#[repr(u8)]
enum ATACommand {
    ReadExt = 0x24,
    Identify = 0xEC
}

#[repr(u8)]
enum ATAFeature {
    PIO = 0x0,
}

#[allow(dead_code)]
#[repr(u8)]
enum ATAStatus {
    ERR = 0b00000001,
    DRQ = 0b00001000,
    DF = 0b00100000,
    RDY = 0b01000000,
    BSY = 0b10000000
}

pub enum ATAError {
    BufferNotAligned,
    BufferOverflow,
    LBATooLarge,
    DiskError(u8),
    DeviceNotExist,
    NotATADevice
}

#[cfg(feature = "alloc")]
impl Into<String> for ATAError {
    fn into(self) -> String {
        match self {
            Self::DiskError(i) => format!("Disk Error: {}", i),
            Self::BufferOverflow => "Disk Error: Overflow".into(),
            Self::LBATooLarge => "Disk Error: LBA too large".into(),
            Self::DeviceNotExist => "Disk Error: not found".into(),
            Self::NotATADevice => "Disk Error: not ATA".into(),
            Self::BufferNotAligned => "Disk Error: alignment".into(),
        }
    }
}

pub enum ATADriver {
    PRIMARY,
    SECONDARY
}

#[allow(dead_code)]
impl ATADriver {
    const fn io_base(&self) -> u16 {
        match self {
            &Self::PRIMARY => 0x1f0,
            &Self::SECONDARY => 0x170
        }
    }

    const fn data_reg(&self) -> u16 { self.io_base() + 0 }
    const fn feature_reg(&self) ->  u16 { self.io_base() + 1 }
    /// for reading
    const fn error_reg(&self) -> u16 { self.io_base() + 1 }
    const fn sector_num_reg(&self) -> u16 { self.io_base() + 2 }
    const fn lba_lo_reg(&self) -> u16 { self.io_base() + 3 }
    const fn lba_mid_reg(&self) -> u16 { self.io_base() + 4 }
    const fn lba_hi_reg(&self) -> u16 { self.io_base() + 5 }/// Used to select a drive and/or head.   Supports extra address/flag bits.
    const fn drive_reg(&self) -> u16 { self.io_base() + 6 }/// for reading
    const fn status_reg(&self) -> u16 { self.io_base() + 7 }/// for writing 
    const fn command_reg(&self) -> u16 { self.io_base() + 7 }

    const fn ctrl_base(&self) -> u16 { 0x376 }
    const fn dcr_reg(&self) -> u16 { self.ctrl_base() + 0 }
    const fn alt_status_reg(&self) -> u16 { self.ctrl_base() + 0 }
    /// drive address register
    const fn dar(&self) -> u16 { self.ctrl_base() + 1 }

    fn ata_delay_400ns(&self) {
        inb(self.status_reg());
        inb(self.status_reg());
        inb(self.status_reg());
        inb(self.status_reg());
    }
}

