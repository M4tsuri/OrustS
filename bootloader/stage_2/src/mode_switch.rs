use core::{
    intrinsics::transmute, 
    marker::PhantomData,
    arch::asm
};
use shared::{
    gdt::{GDTSelector, GDT_TABLE},
    layout::STACK_END
};
use i386::{mem::dt::gdt::GDTDescriptor, instrs::*};

use crate::img_load::STAGE3_PTR;

/// Transfer cpu mode from real mode to protect mode.
/// Protect mode privides us with segmentation of physical address space (also called linear address space), 
/// which allows us to isolate code, data and stack segment from each other and
/// set proper permissions for them.
/// Note that segmentation still operates on physical address space.
/// If we need more powerful virtual address space, we need to activate paging in addition.
///
/// The detailed steps are descripted in
/// *Intel Developer Manual Volume Section 9.9.1 Switching to Protect Mode*.
pub fn to_protect() -> ! {
     // 1. Disable maskable hardware interrupts
    cli();

    unsafe {
        // 2. Execute `lgdt` instruction to load address of GDT to GDTR register.
        //    Here we directly use a externed symbol in instruction, so linker will help 
        //    us relocate it to its real address at compile time
        GDTDescriptor::update(&GDT_TABLE)
            .or(Err("Error when loading GDT.")).unwrap();
        // 3. Set PE flag in control register CR0, which activates segmentation.
        //    If needed, set PG flag for paging.
        //    Set CR0.PG = 1 and CR4.PAE = 0 (origin value) for 32-bit paging.
        //    See *Intel Developer Manual Vol. 3A 4-3*
        asm! {
            "mov eax, cr0",
            "or eax, 1",
            "mov cr0, eax",
            // 5. Load DS, SS, ES, FS and GS with corresponding GDT selectors
            "mov ax, {data}",
            "mov ds, ax",
            "mov es, ax",
            "mov gs, ax",
            "mov ax, {stack}",
            "mov ss, ax",
            "mov esp, {stack_but}",
            "mov ax, {null}",
            "mov fs, ax",
            // 6. re-enable hardware interrupts
            // TODO: Enable hardware interrupt.
            // Currently directly executing sti instruction causes weird behavior of QEMU 
            // due to the lack of IDT.
            // See https://lists.gnu.org/archive/html/qemu-discuss/2015-01/msg00033.html
            //  "sti"
            // 4. Do a far jump to the next instruction to serialize the processer 
            //    (clear the pipeline, I don't know how does this work =-=)
            //    This step also sets the cs register.
            "jmp {CS}, offset {target}",
            data = const GDTSelector::DATA as u16,
            stack = const GDTSelector::STACK as u16,
            null = const GDTSelector::NULL as u16,
            stack_but = const STACK_END - 0x10,
            CS = const GDTSelector::CODE as u16,
            target = sym to_stage3,
            out("eax") _,
        }
    }

    loop {}
}

#[no_mangle]
fn to_stage3() -> ! {
    (unsafe { 
        transmute::<*const PhantomData<()>, fn() -> !>(&STAGE3_PTR as *const PhantomData<()>)
    })()
}
