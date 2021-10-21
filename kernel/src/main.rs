#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[used]
static TMP: [u8; 0x100] = [0; 0x100];

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn main() {
    let a = TMP[3];
    let b = TMP[4];
    if a * a + b * b == a * b {
        let c = Some(a);   
    }
    loop {}
}
