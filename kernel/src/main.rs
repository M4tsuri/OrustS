#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod display;
#[macro_use]
extern crate lazy_static;
use core::{
    panic::PanicInfo,
    arch::asm
};
use i386::utils::u8x::CastUp;
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

/// log some hardware information on screen
fn show_info(ctx: &KernelContext<>) {
    // show memory information
    println!("\nMemory Information: \n");
    println!("    {:<12}{:<12}{:<12}", "Base", "End", "Type");
    ctx.mem_info.get_ranges().unwrap().iter().for_each(|x| {
        let ty: &'static str = x.ty.into();
        println!("    {:<#12x}{:<#12x}{:<12}", x.base, x.base + x.len, ty)
    });

    println!("\n\nDisk Information: \n");
    let max_lba48: u64 = ctx.disk_info.lba48_sec.cast_le();
    println!("    MAX ATA LBA48 SECTORS: {}", max_lba48);
    println!("\n\n");
}

#[link_section = ".startup"]
#[no_mangle]
fn main(ctx: KernelContext) {
    scr_clear();
    println!("[INFO] Kernel Entered.");
    show_info(&ctx);

    loop {}
}
