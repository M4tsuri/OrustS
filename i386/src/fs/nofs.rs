pub mod real;

#[cfg(feature = "alloc")]
pub mod protected;

/// A simple file system we use in our bootloader (and even in kernel currently), 
/// which is no file system.

/// The sector LBA for the file
type NoFSIdent = u64;


