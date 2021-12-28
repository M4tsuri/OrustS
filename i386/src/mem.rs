/// This module contains code for memory management

#[cfg(feature = "alloc")]
pub mod paging;
pub mod dt;

#[cfg(feature = "alloc")]
pub mod info;

/// currently this must be u64, DO NOT change it
pub type PhysAddr = u64;
pub type VirtAddr = usize;