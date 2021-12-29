//! This module contains code for memory management

pub mod paging;
pub mod dt;

use core::ops::Sub;

#[cfg(feature = "alloc")]
pub mod info;

/// currently this must be u64, DO NOT change it
pub type PhysAddr = u64;
pub type VirtAddr = usize;

/// A memory range 
pub struct MemRange<T> {
    pub start: T,
    pub end: T,
    pub len: T,
}

impl<T: Sub<Output = T> + Copy> MemRange<T> {
    pub fn new(start: T, end: T) -> Self {
        Self {
            start, end,
            len: end - start
        }
    }
}
