#![no_std]
#![feature(asm)]
#![feature(slice_as_chunks)]
#![feature(core_intrinsics)]

pub mod dt;
pub mod ring;
pub mod utils;
pub mod tss;
pub mod instrs;
pub mod driver;
pub mod screen;
pub mod fs;
