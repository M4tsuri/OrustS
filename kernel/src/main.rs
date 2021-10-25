#![no_std]
#![no_main]
#![feature(asm)]

use core::panic::PanicInfo;

#[used]
static mut TMP: [u32; 0x100] = [0xdeadbeef; 0x100];

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn main() {
    unsafe {
        asm! {
            "mov word ptr [{}], 0xbadcaffe",
            in(reg) &TMP
        }

        let a = TMP[3];
        let b = TMP[4];
        if a * a + b * b == a * b {
            let c = Some(a);   
        }
    }
    
    loop {}
}
