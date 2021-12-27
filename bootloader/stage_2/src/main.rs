#![no_std]
#![no_main]

#![feature(asm_const)]
#![feature(asm_sym)]

mod mode_switch;
mod display;
mod img_load;
mod a20;

use core::{
    panic::PanicInfo,
    arch::asm
};
use a20::{check_a20, enable_a20};
use display::display_real;
use img_load::load_stage3;
use mode_switch::to_protect;
use shared::mem::MEMINFO;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // FIXME: currently it does not work, I don't know why 
    if let Some(msg) = info.payload().downcast_ref::<&'static str>() {
        display_real(*msg);
    } else {
        display_real("Unknown error in stage 2.");
    }
    unsafe { asm!("hlt") }
    loop {}
}

fn main() {
    display_real("Stage 2 entered.");
    load_stage3().or(Err("Disk Error.")).unwrap();
    display_real("Stage 3 loaded.");

    // try to enable A20 line
    for _ in 0..255 {
        if check_a20() {
            break;
        }
        enable_a20();
    }

    if !check_a20() {
        panic!("A20 not enabled.");
    }

    unsafe {
        MEMINFO.query()
            .ok_or("Error when getting memory info.").unwrap();
    }
}

/// Our entrypoiny of bootloader.
/// The loader will be loaded to 0x7c00 by BIOS, which has been considered by our linker script
#[link_section = ".startup"]
#[no_mangle]
fn _start() -> ! {
    main();
    to_protect()
}
