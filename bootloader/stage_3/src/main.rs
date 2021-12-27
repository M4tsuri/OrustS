#![no_std]
#![no_main]
#![feature(asm)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

mod display;
mod load_kernel;

extern crate alloc;

use core::intrinsics::transmute;
use core::marker::PhantomData;
use core::{alloc::Layout, panic::PanicInfo};
use alloc::string::String;
use display::scr_clear;
use i386::fs::{FSError, nofs::protected::NoFSProtected};
use i386::driver::disk::ata::{ATADriver, ATAError};
use load_kernel::load_kernel;
use shared::kctx::KernelContext;
use shared::mem::MEMINFO;
use static_alloc::Bump;

use crate::load_kernel::KERNEL_PTR;

#[global_allocator]
static ALLOC: Bump<[u8; 1 << 16]> = Bump::uninit();

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(msg) = info.message() {
        println!("Error: {}", msg);
    } else {
        println!("Unknown Error.");
    }
    unsafe { asm!("hlt") }
    loop {}
}

#[alloc_error_handler]
fn oom(_layout: Layout) -> ! {
    println!("Alloc Error.");
    unsafe { asm!("hlt") }
    loop {}
}

/// The main function of stage 3. 
/// This function should collect all possible errors so we can deal with them in _start.
fn main() -> Result<KernelContext, String> {
    let fs = NoFSProtected::new(ATADriver::PRIMARY)
        .map_err(|x| <FSError<ATAError> as Into<String>>::into(x))?;
    load_kernel(&fs)?;
    println!("Kernel loaded.");
    // switch to real mode and poweroff, just for illustrating our mode switching works.
    // crate::mode_switch::to_real(crate::mode_switch::poweroff as u16);
    Ok(KernelContext {
        disk_info: fs.get_disk_info(),
        mem_info: unsafe { MEMINFO.clone() }
    })
}

/// Now we are in protect mode. According to *Intel Developer Manual Vol. 3A 9-13*, 
/// Execution in protect mode begins with a CPL with 0.
#[link_section = ".startup"]
#[no_mangle]
fn _start() -> ! {
    scr_clear();
    
    println!("Loading kernel into RAM...");
    let kernel: fn(KernelContext) -> ! = unsafe { 
        transmute(&KERNEL_PTR as *const PhantomData<()>) 
    };
    kernel(main().unwrap())
}
