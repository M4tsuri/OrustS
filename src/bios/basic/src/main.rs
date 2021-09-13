#![no_std]
#![no_main]
#![feature(asm)]

use core::panic::{self, PanicInfo};

#[link_section = ".startup"]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[used]
#[link_section = ".magic"]
static BIOS_MAGIC: u16 = 0xaa55;

#[link_section = ".startup"]
/// display a string on screen, see https://stanislavs.org/helppc/int_10-13.html
fn display(src: &str) {
    let ptr = src.as_ptr();
    let len = src.len() as u16;

    unsafe {
        asm! {
            "mov eax, {0}",
            "mov bp, ax",
            "mov ecx, {1}",
            "mov ax, 01301h",
            "mov bx, 000ch",
            "mov dl, 0",
            "int 10h",
            in(reg) ptr,
            in(reg) len
        }
    }
}

/// Our entrypoiny of bootloader.
/// The loader will be loaded to 0x7c00 by BIOS, which has been considered by our linker script
/// We firstly initialize 
#[link_section = ".startup"]
#[no_mangle]
fn _start() -> ! {
    unsafe {
        asm!(
            "mov ax, cs",
            "mov ds, ax",
            "mov es, ax",
            "mov esp, 0xff00",
        );
    }
    
    loop {}
}
