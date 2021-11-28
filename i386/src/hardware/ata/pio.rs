use core::mem::size_of;

use super::*;
use crate::{bios::disk::{SECTOR_ALIGN, SECTOR_SIZE}, instrs::{inb, outb, pause}};

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
    pub fn pio_identify(&self) -> Result<[u8; 512], ATAError> {
        let mut result: [u8; 512] = [0; 512];

        let drive = match self {
            ATADriver::PRIMARY => 0xA0,
            ATADriver::SECONDARY => 0xB0
        };

        outb(self.drive(), drive);
        outb(self.sector_num(), 0);
        outb(self.lba_lo(), 0);
        outb(self.lba_mid(), 0);
        outb(self.lba_hi(), 0);

        outb(self.command(), ATACommand::Identify as u8);

        let mut status = inb(self.status());
        if status == 0 {
            return Err(ATAError::DeviceNotExist)
        }

        while status & (ATAStatus::BSY as u8) != 0 {
            pause();
            status = inb(self.status());
        }

        if inb(self.lba_mid()) | inb(self.lba_hi()) != 0 {
            return Err(ATAError::NotATADevice)
        }

        loop {
            if status & (ATAStatus::ERR as u8) != 0 {
                return Err(ATAError::DiskError(inb(self.error())))
            } else if status & (ATAStatus::DRQ as u8) != 0 {
                self.pio_read_port(self.data(), &mut result);
                return Ok(result)
            }
            pause();
            status = inb(self.status());
        }
    }

    pub fn pio_sftrst(&self) {
        outb(self.dcr(), ATADCR::SFTRST as u8);
        outb(self.dcr(), ATADCR::BUSRST as u8);
    
        self.ata_delay_400ns();
    
        let mut status = inb(self.status());
        // spin wait until the BUSY flag is unset and READY flag is set
        while status & ATAStatus::BSY as u8 != 0 || status & ATAStatus::RDY as u8 != 1{
            pause();
            status = inb(self.status());
        }
    }

    pub fn pio_read_sectors(&self, lba: u64, buf: &mut [u8], sec_num: u64) -> Result<(), ATAError> {
        if (buf.len() as u64) < (sec_num << SECTOR_ALIGN) {
            return Err(ATAError::BufferOverflow)
        }
        
        let status = inb(self.alt_status());
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
        let drive = match self {
            ATADriver::PRIMARY => 0x40,
            ATADriver::SECONDARY => 0x50
        };
        // select drive
        outb(self.drive(), drive);
        // set pio mode 
        outb(self.feature(), ATAFeature::PIO as u8);

        // send parameters
        outb(self.sector_num(), ((sec_num >> 8) & 0xff) as u8);
        outb(self.lba_lo(), (lba >> 24 & 0xff) as u8);
        outb(self.lba_mid(), (lba >> 32 & 0xff) as u8);
        outb(self.lba_hi(), (lba >> 48 & 0xff) as u8);
        outb(self.sector_num(), ((sec_num >> 0) & 0xff) as u8);
        outb(self.lba_lo(), (lba >> 0 & 0xff) as u8);
        outb(self.lba_mid(), (lba >> 8 & 0xff) as u8);
        outb(self.lba_hi(), (lba >> 16 & 0xff) as u8);

        // send read sector command
        outb(self.command(), ATACommand::ReadExt as u8);
        // delay 400ns to wait ATA controller to set status registers
        self.ata_delay_400ns();

        let mut status = inb(self.status());
        // spin wait until the BUSY flag is unset
        while status & ATAStatus::BSY as u8 != 0 {
            pause();
            status = inb(self.status());
        }

        // make a error checking
        if status & (ATAStatus::DF as u8 | ATAStatus::ERR as u8) != 0 {
            return Err(ATAError::DiskError(inb(self.error())))
        }

        // now data is ready, we now read data from the data port
        // the remainder must be empty since we had checked the alignment before
        let (sectors, _) = buf.as_chunks_mut::<{SECTOR_SIZE as usize}>();

        for sector in sectors {
            self.pio_read_port(self.data(), sector);
            self.ata_delay_400ns();
            status = inb(self.status());
            if status & (ATAStatus::DF as u8 | ATAStatus::ERR as u8) != 0 {
                return Err(ATAError::DiskError(inb(self.error())))
            }
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


