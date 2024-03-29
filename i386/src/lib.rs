#![no_std]

#![feature(asm_const)]
#![feature(slice_as_chunks)]
#![feature(core_intrinsics)]

pub mod ring;
pub mod utils;
pub mod task;
pub mod instrs;
pub mod driver;
pub mod fs;
pub mod mem;
