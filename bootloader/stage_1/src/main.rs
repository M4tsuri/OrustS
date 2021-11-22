#![no_std]
#![no_main]
#![feature(asm)]

mod img_load;

use core::{intrinsics::transmute, marker::PhantomData, panic::PanicInfo};
use img_load::{STAGE2_PTR, load_stage2};
use i386::bios::video::BIOS_80X25_16_COLOR;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub const TMP_STACK: u16 = 0x7b00;

/// Our entrypoiny of bootloader.
/// The loader will be loaded to 0x7c00 by BIOS, which has been considered by our linker script
#[link_section = ".startup"]
#[no_mangle]
fn _start() -> ! {
    // Initialize all segment registers.
    // There are no concret "segment" now, so just initialize ds, es, ss with 
    // the value ofcode segment.
    // Currently our code and data is mixed, we will change this situation later by entering protect mode
    unsafe {
        asm!(
            "mov ax, cs",
            "mov ds, ax",
            "mov es, ax",
            "mov ss, ax",
            "mov sp, {stack}",
            stack = const TMP_STACK,
            out("ax") _,
        );

        asm! {
            "int 10h",
            in("ax") (0 << 8) | (BIOS_80X25_16_COLOR as u16)
            
        }
    }
    
    if let Err(_) = load_stage2() { 
        unsafe { asm!("hlt") } 
    }
    let stage_2: fn() -> ! = unsafe { transmute(&STAGE2_PTR as *const PhantomData<()>) };
    stage_2()
}
