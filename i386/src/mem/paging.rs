//! This module defines data structures and related utilities about paging.
//! We only support PAE mode paging now.

pub mod pae;

/// supported paging modes
pub enum PagingMode {
    PAE
}

/// This struct configs 
pub struct PagingConfig {
    /// paging mode
    pub mode: PagingMode,
    /// page attribute table 
    pub pat: bool ,
    /// supervisor-mode access prevention
    pub smap: bool,
    /// supervisor-mode execution prevention
    pub smep: bool
}

/// Memory type for caching, this only work if PAT is supported.
/// The combination of these flags creates a 3-bit integer:
/// PAT * 4 + PCD * 2 + PWT.
/// Which is an index into PAT, indicating the memory cache type.
/// FIXME: explicitly set PAT
#[derive(Default)]
pub struct PATMemoryType {
    pat: bool,
    /// page level write-through
    pwt: bool,
    /// page level cache disable
    pcd: bool
}

impl PATMemoryType {
    pub const fn new(pat: bool, pwt: bool, pcd: bool) -> Self {
        Self { pat, pwt, pcd }
    }
}

/// The unified interface for paging modes. every paging mode should implement this trait.
pub trait Paging {
    /// Enter paging mode 
    fn enable(&self);
    /// update the page table to cr3 or related control registers
    fn update(&self);
}
