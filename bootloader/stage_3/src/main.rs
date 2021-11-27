#![no_std]
#![no_main]
#![feature(asm)]

mod display;
mod load_kernel;

use core::panic::PanicInfo;
use display::display_at;
use shared::{layout::STACK_END, gdt::GDTSelector};
use load_kernel::load_kernel;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[inline(always)]
fn init_protect() {
    unsafe {
        // 5. Load DS, SS, ES, FS and GS with corresponding GDT selectors
        asm! {
            "mov ax, {data}",
            "mov ds, ax",
            "mov es, ax",
            "mov gs, ax",
            "mov ax, {stack}",
            "mov ss, ax",
            "mov esp, {stack_but}",
            "mov ax, {null}",
            "mov fs, ax",
            data = const GDTSelector::DATA as u16,
            stack = const GDTSelector::STACK as u16,
            null = const GDTSelector::NULL as u16,
            stack_but = const STACK_END - 0x10,
            out("ax") _
        }

        // 6. re-enable hardware interrupts
        // TODO: Enable hardware interrupt.
        // Currently directly executing sti instruction causes weird behavior of QEMU 
        // due to the lack of IDT.
        // See https://lists.gnu.org/archive/html/qemu-discuss/2015-01/msg00033.html
        // asm!("sti")
    }
}

/// The main function of stage 3. 
/// This function should collect all possible errors so we can deal with them in _start.
/// This function must not be inlined.
#[inline(never)]
fn main() -> Result<(), &'static str> {
    load_kernel()?;
    display_at(9, 0, "Kernel loaded.");
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
    // FIXME: This function call must come before any function prelude, \
    // however currently there is no such guarantee
    init_protect();
    display_at(10, 0, "In Protect Mode Now.");
    
    if let Err(msg) = main() {
        display_at(0, 0, msg);
        unsafe { asm!("hlt"); }
    }
    loop {}
}
