//! This module defines data structures and related utilities about paging.
//! We only support PAE mode paging now.

// pub mod pae;

/// The unified interface for paging modes. every paging mode should implement this trait.
/// V is the type of virtual address, P is the type of physical address.
/// Note that V and P must be unsigned integer.
pub trait Paging<V, P> {
    /// Enter paging mode 
    fn enter();
    /// Convert virtual address to physical address 
    fn virt_to_phys(virt: V) -> P;
    fn phys_to_virt(phys: P) -> V;
}
