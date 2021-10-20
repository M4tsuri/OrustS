use shared::gdt::{GDTSelector, GDT_TABLE};
use i386::{dt::gdt::GDTDescriptor, instrs::*};

/// Transfer cpu mode from real mode to protect mode.
/// Protect mode privides us with segmentation of physical address space (also called linear address space), 
/// which allows us to isolate code, data and stack segment from each other and
/// set proper permissions for them.
/// Note that segmentation still operates on physical address space.
/// If we need more powerful virtual address space, we need to activate paging in addition.
///
/// The detailed steps are descripted in
/// *Intel Developer Manual Volume Section 9.9.1 Switching to Protect Mode*.
pub fn to_protect() -> Result<(), &'static str> {
     // 1. Disable maskable hardware interrupts
    cli();

    unsafe {
        // 2. Execute `lgdt` instruction to load address of GDT to GDTR register.
        //    Here we directly use a externed symbol in instruction, so linker will help 
        //    us relocate it to its real address at compile time
        GDTDescriptor::update(&GDT_TABLE)?;
        
        // 3. Set PE flag in control register CR0, which activates segmentation.
        //    If needed, set PG flag for paging.
        //    Set CR0.PG = 1 and CR4.PAE = 0 (origin value) for 32-bit paging.
        //    See *Intel Developer Manual Vol. 3A 4-3*
        asm! {
            "mov eax, cr0",
            "or eax, 1",
            "mov cr0, eax",
            out("eax") _
        }
    
        // 4. Do a far jump to the next instruction to serialize the processer 
        //    (clear the pipeline, I don't know how does this work =-=)
        //    This step also sets the cs register.
        asm! {
            "jmp {CS}, offset next",
            "next:",
            CS = const GDTSelector::CODE as u16
        }
    }
    Ok(())
}
