//! Basic information about memory.

extern crate alloc;

use alloc::vec::Vec;

use crate::{
    driver::mem::e820::{E820MemInfo, E820MemType},
    mem::{PhysAddr, MemRange}
};

pub struct PhysMemInfo {
    pub segs: Vec<MemRange<PhysAddr>>
}

impl<const MAX: usize> From<E820MemInfo<MAX>> for PhysMemInfo {
    fn from(info: E820MemInfo<MAX>) -> Self {
        Self {
            segs: info.get_ranges().unwrap().into_iter().filter(|x| {
                matches!(x.ty, E820MemType::AddressRangeMemory)
            }).map(|x| {
                MemRange {
                    start: x.base,
                    len: x.len,
                    end: x.base + x.len
                }
            }).collect()
        }
    }
}


