use core::{hint::spin_loop, intrinsics::transmute, mem::size_of};

use super::*;
use crate::{bios::disk::{lba_to_size, slice_as_sectors}, instrs::{inb, outb}, utils::u8x::{Padding, uint}};

/// The disk information read with ATA IDENTIFY command
#[repr(packed)]
pub struct ATADiskInfo {
    _pad0: Padding<{60 * 2}>,
    /// 60th u16
    pub lba28_sec: u32,
    /// 62th u16
    _pad1: Padding<{21 * 2}>,
    /// 83th u16
    pub mode: u16,
    /// 84th u16
    _pad2: Padding<{4 * 2}>,
    /// 88th u16
    pub umda: u16,
    /// 89th u16
    _pad3: Padding<{4 * 2}>,
    /// 93th u16
    pub cable: u16,
    /// 94th u16
    _pad4: Padding<{6 * 2}>,
    /// 100th u16 
    pub lba48_sec: uint<6>,
    /// 103th u16
    _pad5: Padding<{153 * 2}>
    // 256th 
}

pub enum ATAPIOMode {
    PIO28,
    PIO48
}

impl ATADriver {

    /// To use the IDENTIFY command, select a target drive by sending 0xA0 for the 
    /// master drive, or 0xB0 for the slave, to the "drive select" IO port. On the 
    /// Primary bus, this would be port 0x1F6. Then set the Sectorcount, LBAlo, 
    /// LBAmid, and LBAhi IO ports to 0 (port 0x1F2 to 0x1F5). Then send the IDENTIFY 
    /// command (0xEC) to the Command IO port (0x1F7). Then read the Status port 
    /// (0x1F7) again. If the value read is 0, the drive does not exist. For any other 
    /// value: poll the Status port (0x1F7) until bit 7 (BSY, value = 0x80) clears. 
    /// Because of some ATAPI drives that do not follow spec, at this point you need 
    /// to check the LBAmid and LBAhi ports (0x1F4 and 0x1F5) to see if they are 
    /// non-zero. If so, the drive is not ATA, and you should stop polling. Otherwise, 
    /// continue polling one of the Status ports until bit 3 (DRQ, value = 8) sets, 
    /// or until bit 0 (ERR, value = 1) sets.
    /// At that point, if ERR is clear, the data is ready to read from the Data 
    /// port (0x1F0). Read 256 16-bit values, and store them.
    pub fn pio_identify(&self) -> Result<ATADiskInfo, ATAError> {
        let mut result: [u8; 512] = [0; 512];

        let drive = match self {
            ATADriver::PRIMARY => 0xA0,
            ATADriver::SECONDARY => 0xB0
        };

        outb(self.drive_reg(), drive);
        outb(self.sector_num_reg(), 0);
        outb(self.lba_lo_reg(), 0);
        outb(self.lba_mid_reg(), 0);
        outb(self.lba_hi_reg(), 0);

        outb(self.command_reg(), ATACommand::Identify as u8);

        let mut status = inb(self.status_reg());
        if status == 0 {
            return Err(ATAError::DeviceNotExist)
        }

        while status & (ATAStatus::BSY as u8) != 0 {
            spin_loop();
            status = inb(self.status_reg());
        }

        if inb(self.lba_mid_reg()) | inb(self.lba_hi_reg()) != 0 {
            return Err(ATAError::NotATADevice)
        }

        loop {
            if status & (ATAStatus::ERR as u8) != 0 {
                return Err(ATAError::DiskError(inb(self.error_reg())))
            } else if status & (ATAStatus::DRQ as u8) != 0 {
                self.pio_read_port(self.data_reg(), &mut result);
                return Ok(unsafe { transmute(result) })
            }
            spin_loop();
            status = inb(self.status_reg());
        }
    }

    pub fn pio_sftrst(&self) {
        outb(self.dcr_reg(), ATADCR::SFTRST as u8);
        outb(self.dcr_reg(), ATADCR::BUSRST as u8);
    
        self.ata_delay_400ns();
    
        let mut status = inb(self.status_reg());
        // spin wait until the BUSY flag is unset and READY flag is set
        while status & ATAStatus::BSY as u8 != 0 || status & ATAStatus::RDY as u8 != 1{
            spin_loop();
            status = inb(self.status_reg());
        }
    }

    pub fn pio_read_sectors(&self, lba: u64, buf: &mut [u8], sec_num: u64) -> Result<(), ATAError> {
        if (buf.len() as u64) < lba_to_size(sec_num) {
            return Err(ATAError::BufferOverflow)
        }
        
        let status = inb(self.alt_status_reg());
        // the previous sould have properly cleared BSY and DRQ
        if status & (ATAStatus::BSY as u8 | ATAStatus::DRQ as u8) != 0 {
            self.pio_sftrst();
        }

        if (lba + (buf.len() as u64 >> 9)) >> 33 == 0 {
            self.pio48_read_sectors(lba, buf, sec_num)
        } else {
            return Err(ATAError::LBATooLarge)
        }
        
    }

    /// read sectors in PIO mode, this has huge performance issue. So it should only be used
    /// in bootloader for loading kernel into memory.
    fn pio48_read_sectors(&self, lba: u64, buf: &mut [u8], sec_num: u64) -> Result<(), ATAError> {
        // now data is ready, we now read data from the data port
        // the remainder must be empty since we had checked the alignment before
        let sectors = slice_as_sectors(buf)
            .ok_or(ATAError::BufferNotAligned)?;

        if sec_num > sectors.len() as u64 {
            return Err(ATAError::BufferOverflow)
        }

        let drive = match self {
            ATADriver::PRIMARY => 0x40,
            ATADriver::SECONDARY => 0x50
        };

        
        // select drive
        outb(self.drive_reg(), drive);
        // set pio mode 
        outb(self.feature_reg(), ATAFeature::PIO as u8);

        // send parameters
        outb(self.sector_num_reg(), ((sec_num >> 8) & 0xff) as u8);
        outb(self.lba_lo_reg(), (lba >> 24 & 0xff) as u8);
        outb(self.lba_mid_reg(), (lba >> 32 & 0xff) as u8);
        outb(self.lba_hi_reg(), (lba >> 48 & 0xff) as u8);
        outb(self.sector_num_reg(), ((sec_num >> 0) & 0xff) as u8);
        outb(self.lba_lo_reg(), (lba >> 0 & 0xff) as u8);
        outb(self.lba_mid_reg(), (lba >> 8 & 0xff) as u8);
        outb(self.lba_hi_reg(), (lba >> 16 & 0xff) as u8);

        // send read sector command
        outb(self.command_reg(), ATACommand::ReadExt as u8);

        for sector in &mut sectors[..sec_num as usize] {
            // delay 400ns to wait ATA controller to set status registers
            self.ata_delay_400ns();

            let mut status = inb(self.status_reg());
            // spin wait until the BUSY flag is unset
            while status & ATAStatus::BSY as u8 != 0 {
                spin_loop();
                status = inb(self.status_reg());
            }
        
            // make a error checking
            if status & (ATAStatus::DF as u8 | ATAStatus::ERR as u8) != 0 {
                return Err(ATAError::DiskError(inb(self.error_reg())))
            }
            self.pio_read_port(self.data_reg(), sector);
        }
        
        Ok(())
    }

    /// MAKE SURE SIZE IS EVEN!!!
    fn pio_read_port<const SIZE: usize>(&self, port: u16, buf: &mut [u8; SIZE]) {
        unsafe {
            asm! {
                "rep insw",
                // BE AWARE THAT REP MODIFIES ECX AND EDI!!!
                inout("ecx") SIZE / size_of::<u16>() => _,
                in("dx") port,
                inout("edi") buf => _
            }
        }
    }
}


