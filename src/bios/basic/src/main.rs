#![no_std]
#![no_main]
#![feature(asm)]

mod mode_switch;
mod display;
mod bitwise;

use core::panic::PanicInfo;
use display::display;
use mode_switch::to_protect;

#[link_section = ".stage_1"]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[used]
#[link_section = ".magic"]
static BIOS_MAGIC: u16 = 0xaa55;

/// Our entrypoiny of bootloader.
/// The loader will be loaded to 0x7c00 by BIOS, which has been considered by our linker script
#[link_section = ".startup"]
#[no_mangle]
unsafe fn _start() -> ! {
    asm!(
        "mov ax, cs",
        "mov ds, ax",
        "mov es, ax",
        "mov sp, 0xff00",
    );

    to_protect();

    loop {}
}
