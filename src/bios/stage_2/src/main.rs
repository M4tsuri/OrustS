#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[link_section = ".stage_2"]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}