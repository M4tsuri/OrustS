pub const SECTOR_SIZE: u32 = 512;
pub const SECTOR_ALIGN: u16 = 9;

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