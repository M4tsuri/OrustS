use crate::instrs::inb;

/// Support for some useful ATA PIO commands
/// See https://wiki.osdev.org/ATA_PIO_Mode

pub mod pio;

const ATA_CMD_PACKET: u8 = 0xa0;
const ATA_CMD_READ_EXT: u8 = 0x24;

const ATA_FEATURE_PIO: u8 = 0x0;

const ATA_STATUS_ERR: u8 = 0b00000001;
const ATA_STATUS_DRQ: u8 = 0b00001000;
const ATA_STATIS_DF:  u8 = 0b00100000;
const ATA_STATUS_RDY: u8 = 0b01000000;
const ATA_STATUS_BSY: u8 = 0b10000000;



#[derive(Clone, Copy)]
#[repr(u16)]
pub enum ATA_BUS {
    PRIMARY = 0x1F0,
    SECONDARY = 0x170
}

#[repr(u8)]
pub enum ATA_DRIVE28 {
    PRIMARY = 0xA0,
    SECONDARY = 0xB0
}

#[repr(u8)]
pub enum ATA_DRIVE48 {
    PRIMARY = 0x50,
    SECONDARY = 0x40
}

#[inline]
const fn data_reg(bus: ATA_BUS) -> u16 {
    bus as u16 + 0
}

/// for writing 
#[inline]
const fn feature_reg(bus: ATA_BUS) -> u16 {
    bus as u16 + 1
}

/// for reading
#[inline]
const fn error_reg(bus: ATA_BUS) -> u16 {
    bus as u16 + 1
}

#[inline]
const fn sector_num_reg(bus: ATA_BUS) -> u16 {
    bus as u16 + 2
}

#[inline]
const fn lba_lo_reg(bus: ATA_BUS) -> u16 {
    bus as u16 + 3
}

#[inline]
const fn lba_mid_reg(bus: ATA_BUS) -> u16 {
    bus as u16 + 4
}

#[inline]
const fn lba_hi_reg(bus: ATA_BUS) -> u16 {
    bus as u16 + 5
}

/// Used to select a drive and/or head. Supports extra address/flag bits.
#[inline]
const fn drive_reg(bus: ATA_BUS) -> u16 {
    bus as u16 + 6
}

/// for reading
#[inline]
const fn status_reg(bus: ATA_BUS) -> u16 {
    bus as u16 + 7
}

/// for writing 
#[inline]
const fn command_reg(bus: ATA_BUS) -> u16 {
    bus as u16 + 7
}

#[inline]
const fn dcr_reg(bus: ATA_BUS) -> u16 {
    bus as u16 + 0x206
}
