#![no_std]
#![no_main]
#![feature(asm)]

mod display;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[used]
static mut MAGIC: [u8; 0x400] = [0xcc; 0x400];

#[no_mangle]
fn main() {
    unsafe {
        asm! {
            "mov dword ptr [edx + 0x12], 0xdeadbeef",
            in("edx") &MAGIC
        }
    }
    loop {}
}
