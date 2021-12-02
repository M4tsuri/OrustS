#![no_std]
#![no_main]
#![feature(asm)]
#![feature(panic_info_message)]

mod display;
#[macro_use]
extern crate lazy_static;
use core::panic::PanicInfo;

use shared::kctx::KernelContext;

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

#[used]
static mut MAGIC: [u8; 0x400] = [0xcc; 0x400];

#[link_section = ".startup"]
#[no_mangle]
fn main(ctx: KernelContext) {
    println!("Kernel entered.");
    loop {}
}
