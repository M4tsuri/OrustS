use i386::ring::Privilege;
use i386::gdt_ldt::{GDTDescriptor, pack_gdt, GDTSelector};

/// the length of GDT, 5 by default (include a null entry)
const GDT_LEN: u16 = 5;

/// The GDT, Global Descriptor Table.
/// The address of GDT should be 8 byte aligned to get better performance (see *Intel Developer Manual Vol. 3A 3-15*).
#[used]
#[link_section = ".gdt"]
static GDT_TABLE: [u64; GDT_LEN as usize] = [
    // An empty entry (Null Segment) which is reserved by Intel
    pack_gdt(0, 0, 0, 0, 
        Privilege::Ring0 as u8, 0, 0, 0), 
    // Code Segment, 512KiB
    pack_gdt(0x0, 0x80000, 8, 1, 
        Privilege::Ring0 as u8, 1, 0b100, 0),
    // Data Segment, 112KiB
    pack_gdt(0x80000, 0x9c000, 3, 1, 
        Privilege::Ring0 as u8, 1, 0b100, 0),
    // Stack Segment, 112KiB, grow down
    pack_gdt(0x9c000, 0xb8000, 7, 1, 
        Privilege::Ring0 as u8, 1, 0b100, 0),
    // Video RAM
    pack_gdt(0xb8000, 0xffff, 3, 1, 
        Privilege::Ring0 as u8, 1, 0b100, 0), 
];

/// An instance of GDT descriptor, occupying 6 bytes in memory.
/// The `limit` field is the length of GDT **in bytes** - 1, which is used by processor 
/// to find the last valid byte in GDT (see *Intel Developer Manual Vol. 3A 3-15*).
#[used]
#[no_mangle]
#[allow(improper_ctypes)]
#[link_section = ".gdt_desc"]
pub static mut GDT_DESCRIPTOR: GDTDescriptor = GDTDescriptor {
    limit: GDT_LEN * 8 - 1,
    base_address: GDT_TABLE.as_ptr()
};

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

    // 5. Load DS, SS, ES, FS and GS with corresponding GDT selectors
    asm! {
        "next:",
        "mov ax, {data}",
        "mov ds, ax",
        "mov ax, {stack}",
        "mov ss, ax",
        "mov ax, {null}",
        "mov es, ax",
        "mov fs, ax",
        "mov ax, {video}",
        "mov gs, ax",
        data = const GDTSelector::DATA as u16,
        stack = const GDTSelector::STACK as u16,
        null = const GDTSelector::NULL as u16,
        video = const GDTSelector::VIDEO as u16
    }

    // 6. re-enable hardware interrupts
    asm!("sti")
}
