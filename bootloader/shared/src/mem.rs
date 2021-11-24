/// This module provides support for reading memory information

use core::mem::size_of;


use crate::layout::*;
use i386::bios::mem::{E820MemInfo, E820MemRange, E820MemType};

/// the maximum number of meminfo struct
const MEMINFO_MAX: usize = MEMINFO_SIZE / size_of::<E820MemRange>();

#[link_section = ".meminfo"]
pub static mut _MEMINFO: [E820MemRange; MEMINFO_MAX] = [E820MemRange {
    base: 0,
    len: 0,
    ty: E820MemType::Undefined
}; MEMINFO_MAX];

pub static mut MEMINFO: Option<E820MemInfo> = None;
