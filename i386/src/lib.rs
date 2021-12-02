#![no_std]
#![feature(asm)]
#![feature(slice_as_chunks)]
#![feature(core_intrinsics)]

pub mod dt;
pub mod ring;
pub mod bios;
pub mod utils;
pub mod tss;
pub mod instrs;
pub mod hardware;
pub mod screen;
pub mod fs;
