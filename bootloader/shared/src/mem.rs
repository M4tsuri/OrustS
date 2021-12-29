//! This module provides support for reading memory information

use core::mem::size_of;
use i386::driver::mem::e820::{E820MemInfo, E820MemRange};

use crate::layout::*;


/// the maximum number of meminfo struct
pub const MEMINFO_MAX: usize = MEMINFO_SIZE / (size_of::<E820MemRange>() + 4);

#[link_section = ".meminfo"]
pub static mut MEMINFO: E820MemInfo<MEMINFO_MAX> = E820MemInfo::new();
