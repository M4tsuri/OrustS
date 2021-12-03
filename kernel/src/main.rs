#![no_std]
#![no_main]
#![feature(asm)]
#![feature(panic_info_message)]

mod display;
#[macro_use]
extern crate lazy_static;
use core::panic::PanicInfo;

use shared::kctx::KernelContext;

use crate::display::scr_clear;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(msg) = info.message() {
        println!("Error: {}", msg);
    } else {
        println!("Unknown Error.");
    }
    unsafe { asm!("hlt") }
    loop {}
}

#[link_section = ".startup"]
#[no_mangle]
fn main(ctx: KernelContext) {
    scr_clear();
    println!("Kernel entered.");
    loop {}
}
