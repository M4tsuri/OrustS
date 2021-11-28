#![no_std]
#![no_main]
#![feature(asm)]
#![feature(const_slice_from_raw_parts)]

mod display;
mod load_kernel;

use core::panic::PanicInfo;
use display::{print, scr_clear};
use load_kernel::load_kernel;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// The main function of stage 3. 
/// This function should collect all possible errors so we can deal with them in _start.
/// This function must not be inlined.
#[inline(never)]
fn main() -> Result<(), &'static str> {
    load_kernel()?;
    print("Kernel loaded.");
    // switch to real mode and poweroff, just for illustrating our mode switching works.
    // crate::mode_switch::to_real(crate::mode_switch::poweroff as u16);
    Ok(())
}

/// Now we initially entered. According to *Intel Developer Manual Vol. 3A 9-13*, 
/// Execution in protect mode begins with a CPL with 0.
/// Note that this function cannot have local varibales because we manually set esp
#[link_section = ".startup"]
#[no_mangle]
fn _start() -> ! {
    scr_clear();
    
    if let Err(msg) = main() {
        print(msg);
        unsafe { asm!("hlt"); }
    }
    loop {}
}
