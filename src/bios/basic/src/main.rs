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

/// Initialize all segment registers.
/// There are no concret "segment" now, so just initialize ds, es, ss with 
/// the value ofcode segment.
/// Currently our code and data is mixed, we will change this situation later by entering 
/// protect mode
#[inline]
unsafe fn init_regs() {
    asm!(
        "mov ax, cs",
        "mov ds, ax",
        "mov es, ax",
        "mov ss, ax"
    );
}

/// Our entrypoiny of bootloader.
/// The loader will be loaded to 0x7c00 by BIOS, which has been considered by our linker script
#[link_section = ".startup"]
#[no_mangle]
unsafe fn _start() -> ! {
    init_regs();
    to_protect();

    loop {}
}
