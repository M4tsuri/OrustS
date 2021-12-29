//! The simple file system we use in our bootloader (and even in kernel currently), 
//! which is no file system.

pub mod real;

#[cfg(feature = "alloc")]
pub mod protected;

/// The sector LBA for the file
type NoFSIdent = u64;


