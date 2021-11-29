const SECTOR_SIZE: u32 = 512;
const SECTOR_ALIGN: u16 = 9;

/// This module is used for some disk read/write functionalities.

const MAX_READ_BYTES: u32 = 0x10000;
const MAX_READ_SECTORS: u16 = (MAX_READ_BYTES >> SECTOR_ALIGN as u32) as u16;

#[inline]
pub const fn size_to_lba(src: usize) -> u64 {
    (src >> SECTOR_ALIGN) as u64
}

#[inline]
pub const fn lba_to_size(lba: u64) -> u64 {
    lba << SECTOR_ALIGN
}

#[inline]
pub const fn is_sector_aligned(src: usize) -> bool {
    src & (SECTOR_SIZE as usize - 1) == 0
}

pub fn slice_as_sectors<'a>(src: &'a mut [u8]) -> Option<&'a mut [[u8; SECTOR_SIZE as usize]]> {
    if !is_sector_aligned(src.len()) {
        return None
    }

    let (res, _) = src.as_chunks_mut::<{SECTOR_SIZE as usize}>();
    Some(res)
}

pub enum DAPError {
    DiskError
}

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

#[inline(always)]
fn extended_read_sectors(disk: u8, dap_ptr: *const DAP) -> Result<(), DAPError> {
    let mut res: u16;
    unsafe {
        asm! {
            "push si",
            "mov si, {si:x}",
            "int 0x13",
            "pop si",
            si = in(reg) dap_ptr,
            inout("ax") 0x4200_u16 => res,
            in("dl") disk,
        }
    }
    if res >> 8 == 0 {
        Ok(())
    } else {
        Err(DAPError::DiskError)
    }
}

pub fn read_disk(addr: (u8, u64), buffer: &mut [u8]) -> Result<(), DAPError> {
    let dest = buffer.as_ptr() as usize;
    let dap = DAP::new(addr, (dest as u16, ((dest >> 16) as u16) << 12), buffer.len());
    dap.read()?;
    Ok(())
}

pub fn reset_disk(disk_id: u8) -> Result<(), DAPError> {
    let mut res: u16;
    unsafe {
        asm! {
            "int 13h",
            in("dl") disk_id,
            inout("ax") 0_u16 => res
        }
    }
    if res >> 8 == 0 {
        Ok(())
    } else {
        Err(DAPError::DiskError)
    }
}

impl DAP {
    /// - disk: (disk_id, start_lba)
    /// - buffer: (segment, offset)
    /// - len: length in bytes
    pub const fn new(disk: (u8, u64), buffer: (u16, u16), len: usize) -> Self {
        Self {
            self_size: 0x10,
            reserved: 0,
            sector_num: (len >> (SECTOR_ALIGN as u32)) as u16,
            buffer_ptr: buffer,
            start_lba: disk.1,
            disk_id: disk.0
        }
    }

    #[inline(always)]
    pub fn read(mut self) -> Result<(), DAPError> {
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
