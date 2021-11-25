use core::mem::size_of;

use super::*;
use crate::{bios::disk::{SECTOR_ALIGN, SECTOR_SIZE}, instrs::{inb, outb, pause}};

#[inline]
fn ata_delay_400ns(bus: ATA_BUS) {
    inb(dcr_reg(bus));
    inb(dcr_reg(bus));
    inb(dcr_reg(bus));
    inb(dcr_reg(bus));
}

pub enum ATA48Err {
    BufferNotAligned,
    LBATooLarge,
    DiskError
}

/// read sectors in PIO mode, this has huge performance issue. So it should only be used
/// in bootloader for loading kernel into memory.
pub fn ata48_pio_read_sectors(bus: ATA_BUS, drive: ATA_DRIVE48, lba: u32, buf: &mut [u8]) -> Result<(), ATA48Err> {
    if buf.len() as u32 & (SECTOR_SIZE - 1) != 0 {
        return Err(ATA48Err::BufferNotAligned)
    }

    if lba >> 33 != 0 {
        return Err(ATA48Err::LBATooLarge)
    }

    let sector_num = (buf.len() as u32 >> SECTOR_ALIGN) as u16;
    // select drive
    outb(drive_reg(bus), drive as u8 & (1 << 4));
    // delay 400ns to wait ATA controller to set status registers
    ata_delay_400ns(bus);
    // set pio mode 
    outb(feature_reg(bus), ATA_FEATURE_PIO);

    // send parameters
    outb(sector_num_reg(bus), ((sector_num >> 8) & 0xff) as u8);
    outb(lba_lo_reg(bus), (lba >> 24 & 0xff) as u8);
    outb(lba_mid_reg(bus), (lba >> 32 & 0xff) as u8);
    outb(lba_hi_reg(bus), (lba >> 48 & 0xff) as u8);
    outb(sector_num_reg(bus), ((sector_num >> 0) & 0xff) as u8);
    outb(lba_lo_reg(bus), (lba >> 0 & 0xff) as u8);
    outb(lba_mid_reg(bus), (lba >> 8 & 0xff) as u8);
    outb(lba_hi_reg(bus), (lba >> 16 & 0xff) as u8);

    // send read sector command
    outb(command_reg(bus), ATA_CMD_READ_EXT);

    ata_delay_400ns(bus);

    let mut status = inb(status_reg(bus));
    // spin wait until the BUSY flag is unset
    while status & ATA_STATUS_BSY != 0 {
        pause();
        status = inb(status_reg(bus));
    }

    // make a error checking
    if status & (ATA_STATIS_DF | ATA_STATUS_ERR) != 0 {
        return Err(ATA48Err::DiskError)
    }

    // now data is ready, we now read data from the data port
    // the remainder must be empty since we had checked the alignment before
    let (sectors, _) = buf.as_chunks_mut::<{SECTOR_SIZE as usize}>();

    for sector in sectors {
        read_sector_from_data_reg(bus, sector);
        ata_delay_400ns(bus);
        if status & (ATA_STATIS_DF | ATA_STATUS_ERR) != 0 {
            return Err(ATA48Err::DiskError)
        }
    }
    Ok(())
}

fn read_sector_from_data_reg(bus: ATA_BUS, buf: &mut [u8; SECTOR_SIZE as usize]) {
    unsafe {
        asm! {
            "mov ecx, {LOOP}",
            "rep insw",
            LOOP = const SECTOR_SIZE as usize / size_of::<u16>(),
            in("dx") data_reg(bus),
            in("edi") buf
        }
    }
}

