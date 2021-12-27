/// This module contains code for memory management

#[cfg(feature = "alloc")]
pub mod paging;
pub mod dt;

#[cfg(feature = "alloc")]
pub mod info;
