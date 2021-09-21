#![no_std]
#![no_main]
#![feature(asm)]

mod display;
mod mem;

use core::panic::PanicInfo;
use display::display_at;
use i386::gdt_ldt::GDTSelector;

#[link_section = ".stage_3"]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[inline]
unsafe fn init_protect() {
    // 5. Load DS, SS, ES, FS and GS with corresponding GDT selectors
    asm! {
        "next:",
        "mov ax, {data}",
        "mov ds, ax",
        "mov ax, {stack}",
        "mov ss, ax",
        "mov ax, {null}",
        "mov es, ax",
        "mov fs, ax",
        "mov ax, {video}",
        "mov gs, ax",
        data = const GDTSelector::DATA as u16,
        stack = const GDTSelector::STACK as u16,
        null = const GDTSelector::NULL as u16,
        video = const GDTSelector::VIDEO as u16
    }

    // 6. re-enable hardware interrupts
    // asm!("sti")
}

#[link_section = ".startup"]
#[no_mangle]
unsafe fn _start() -> ! {
    init_protect();
    display_at(10, 0, "In Protect Mode Now.");
    loop {}
}
