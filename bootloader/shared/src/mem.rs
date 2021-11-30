/// This module provides support for reading memory information

use core::mem::size_of;


use crate::layout::*;
use i386::bios::mem::{E820MemInfo, E820MemRange};

/// the maximum number of meminfo struct
const MEMINFO_MAX: usize = MEMINFO_SIZE / (size_of::<E820MemRange>() + 4);

#[link_section = ".meminfo"]
pub static mut MEMINFO: E820MemInfo<MEMINFO_MAX> = E820MemInfo::new();
