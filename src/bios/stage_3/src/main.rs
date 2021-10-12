#![no_std]
#![no_main]
#![feature(asm)]

mod display;
mod mode_switch;
mod task;

use core::panic::PanicInfo;
use display::display_at;
use i386::{gdt::GDTSelector, ring::Privilege};
use layout::STACK_END;
use task::Task;

#[link_section = ".stage_3"]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[inline]
fn init_protect() {
    unsafe {
        // 5. Load DS, SS, ES, FS and GS with corresponding GDT selectors
        asm! {
            "next:",
            "mov ax, {data}",
            "mov ds, ax",
            "mov ax, {stack}",
            "mov ss, ax",
            "mov esp, {stack_but}",
            "mov ax, {null}",
            "mov es, ax",
            "mov fs, ax",
            "mov ax, {video}",
            "mov gs, ax",
            data = const GDTSelector::DATA as u16,
            stack = const GDTSelector::STACK as u16,
            null = const GDTSelector::NULL as u16,
            video = const GDTSelector::VIDEO as u16,
            stack_but = const STACK_END - 0x10
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
fn main() -> Result<(), &'static str> {
    // just a test here
    let mut task = Task::new(Privilege::Ring0, tmp as u32, 0x1000);
    task.init_ldt()?;
    task.transfer();
    Ok(())
}

/// Now we initially entered. According to *Intel Developer Manual Vol. 3A 9-13*, 
/// Execution in protect mode begins with a CPL with 0.
#[link_section = ".startup"]
#[no_mangle]
fn _start() -> ! {
    init_protect();
    display_at(10, 0, "In Protect Mode Now.");

    // switch to real mode and poweroff, just for illustrating our mode switching works.
    // crate::mode_switch::to_real(crate::mode_switch::poweroff as u16);
    
    if let Err(msg) = main() {
        display_at(0, 0, msg);
        unsafe { asm!("hlt"); }
    }
    loop {}
}

/// A function for testing task switching, just print a "L" at row 10, col 0.
fn tmp() -> ! {
    display_at(10, 0, "L");
    loop {}
}
