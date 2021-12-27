//! Basic information about memory.

extern crate alloc;
use core::marker::PhantomData;

use alloc::vec::Vec;

/// A memory range 
pub struct MemRange<T> {
    start: usize,
    end: usize,
    len: usize,
    meta: T
}

pub struct PhysMemInfo {
    segs: Vec<MemRange<PhantomData<()>>>
}