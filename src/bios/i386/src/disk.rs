pub const SECTOR_SIZE: u32 = 512;
pub const SECTOR_ALIGN: u16 = 9;

/// This module is used for some disk read/write functionalities.

const MAX_READ_BYTES: u32 = 0x10000;
const MAX_READ_SECTORS: u16 = (MAX_READ_BYTES >> SECTOR_ALIGN as u32) as u16;

/// https://en.wikipedia.org/wiki/INT_13H#INT_13h_AH=42h:_Extended_Read_Sectors_From_Drive
#[repr(C, packed)]
pub struct DAP {
    /// size of DAP
    self_size: u8,
    reserved: u8,
    sector_num: u16,
    /// This pointer should obey the form segment:offset.
    /// On x86, offset comes before segment. 
    /// Address = Segment << 4 + Offset (real mode)
    buffer_ptr: (u16, u16),
    start_lba: u64,
    disk_id: u8
}

#[inline]
fn extended_read_sectors(disk: u8, dap_ptr: *const DAP) -> Result<(), &'static str> {
    let mut res: u8;
    unsafe {
        asm! {
            "mov ah, 0x42",
            "mov si, bx",
            "int 0x13",
            in("dl") disk,
            in("bx") dap_ptr,
            out("ah") res
        }
    }
    if res == 0 {
        Ok(())
    } else {
        Err("Disk Error.\n")
    }
}

impl DAP {
    /// - disk: (disk_id, start_lba)
    /// - buffer: (segment, offset)
    /// - len: length in bytes
    pub const fn new(disk: (u8, u64), buffer: (u16, u16), len: u32) -> Self {
        Self {
            self_size: 0x10,
            reserved: 0,
            sector_num: (len >> (SECTOR_ALIGN as u32)) as u16,
            buffer_ptr: buffer,
            start_lba: disk.1,
            disk_id: disk.0
        }
    }

    #[inline]
    pub fn reset(&self) -> Result<(), &'static str> {
        unsafe {
            asm! {
                "xor ah, ah",
                "int 13h",
                in("dl") self.disk_id
            }
        }
        Ok(())
    }

    pub fn read(mut self) -> Result<(), &'static str> {
        if self.sector_num > MAX_READ_SECTORS {
            let remained_sectors = self.sector_num - MAX_READ_SECTORS;
            self.sector_num = MAX_READ_SECTORS as u16;
            extended_read_sectors(self.disk_id, &self as *const DAP)?;

            self.buffer_ptr.1 += 0x1000;
            self.start_lba += MAX_READ_SECTORS as u64;
            self.sector_num = remained_sectors;
            self.read()
        } else {
            extended_read_sectors(self.disk_id, &self as *const DAP)?;
            Ok(())
        }
    }
}
