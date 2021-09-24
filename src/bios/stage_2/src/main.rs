#![no_std]
#![no_main]
#![feature(asm)]

mod mode_switch;
mod display;
mod img_load;

use core::{marker::PhantomData, panic::PanicInfo, mem::transmute};
use display::display_real;
use img_load::{STAGE3_PTR, load_stage3};
use mode_switch::to_protect;


#[link_section = ".stage_2"]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Our entrypoiny of bootloader.
/// The loader will be loaded to 0x7c00 by BIOS, which has been considered by our linker script
#[link_section = ".startup"]
#[no_mangle]
fn _start() -> ! {
    display_real("Stage 2 entered.");
    if let Err(msg) = load_stage3() {
        display_real(msg);
    }
    display_real("Stage 3 loaded.");
    to_protect();
    
    let stage_3: fn() -> ! = unsafe { transmute(&STAGE3_PTR as *const PhantomData<()>) };
    stage_3()
}
