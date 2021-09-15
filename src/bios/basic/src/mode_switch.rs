use crate::gdt_ldt::{GDT_DESCRIPTOR, GDTSelector};

/// Transfer cpu mode from real mode to protect mode.
/// Protect mode privides us with segmentation of physical address space (also called linear address space), 
/// which allows us to isolate code, data and stack segment from each other and
/// set proper permissions for them.
/// Note that segmentation still operates on physical address space.
/// If we need more powerful virtual address space, we need to activate paging in addition.
///
/// The detailed steps are descripted in
/// *Intel Developer Manual Volume Section 9.9.1 Switching to Protect Mode*.
#[link_section = ".stage_1"]
#[inline]
pub unsafe fn to_protect() {
    // 1. Disable maskable hardware interrupts
    asm!("cli");
    
    // 2. Execute `lgdt` instruction to load address of GDT to GDTR register.
    //    Here we directly use a externed symbol in instruction, so linker will help 
    //    us relocate it to its real address at compile time
    asm!("lgdt {}", sym GDT_DESCRIPTOR);

    // 3. Set PE flag in control register CR0, which activates segmentation.
    //    If needed, set PG flag for paging.
    asm! {
        "mov eax, cr0",
        "or eax, 1",
        "mov cr0, eax"
    }

    // 4. Do a far jump to the next instruction to serialize the processer 
    //    (clear the pipeline, I don't know how does this work =-=)
    //    This step also sets the cs register.
    asm! {
        "jmp {}, offset next", const GDTSelector::CODE as u16
    }

    // 5. jump to code/data segment selector
    asm! {
        "next:",
    }
}

