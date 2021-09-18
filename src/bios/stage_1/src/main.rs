#![no_std]
#![no_main]
#![feature(asm)]

mod img_load;

use core::{intrinsics::transmute, marker::PhantomData, panic::PanicInfo};
use img_load::{STAGE2_PTR, load_stage2};

#[link_section = ".stage_1"]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[used]
#[link_section = ".magic"]
static BIOS_MAGIC: u16 = 0xaa55;

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
            "mov ax, 0x0003",
            "int 10h",
            stack = const TMP_STACK
        );
    }
    
    if let Err(_) = load_stage2() { 
        unsafe { asm!("hlt") } 
    }
    let stage_2: fn() -> ! = unsafe { transmute(&STAGE2_PTR as *const PhantomData<()>) };
    stage_2()
}
