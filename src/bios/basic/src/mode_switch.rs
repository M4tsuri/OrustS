use crate::bitwise::mask_assign;

/// the length of GDT, 3 by default (include a null entry)
const GDT_LEN: u16 = 5;

/// A GDT Descriptor descriping the length of GDT and location of GDT in memory.
/// The address of this describtor will be passed to lgdt instruction to fill GDT.
///
/// The `limit` field is the length of GDT **in bytes** - 1, which is used by processor 
/// to find the last valid byte in GDT (see *Intel Developer Manual Vol. 3A 3-15*).
#[repr(packed)]
#[allow(improper_ctypes)]
struct GDTDescriptor {
    limit: u16,
    base_address: *const u64
}

unsafe impl Sync for GDTDescriptor {}

/// Pack data in parameter to a valid GDT entry.
/// A GDT entry can be represented as a 64 bit value, whose fields are defined as follows:
///
/// ```text
/// | 0:16  | limit[0:16]     | segment size limit          |
/// | 16:32 | base[0:16]      | segment base address        |
/// | 32:40 | base[16:24]     |                             |
/// | 40:44 | type[0:4]       | segment type and attributes |
/// | 44:45 | s[0:1]          | system or data/code segment |
/// | 45:47 | privilege[0:2]  | 0 = Kernel, 3 = User        |
/// | 47:48 | present[0:1]    | 1 = enable segment          |
/// | 48:52 | limit[16:20]    |                             |
/// | 52:55 | attributes[0:3] | segment attributes          |
/// | 55:56 | granularity     | alignment                   |
/// | 56:64 | base[24:32]     |                             |
/// ```
///
/// For the type field, see *Intel Developer Manual 3-12 Vol.3A Table 3-1*
///
/// For the s field: clear if this is a system segment, set if this is a code/data segment 
///
/// For the attributes field: 
///
/// - 0: Available to System Programmers flag, reserved
/// - 1: 64-bit code segment
/// - 2: size bit, set if out code is 32-bit, 16-bit vice versa
///
/// For granularity, CPU will multiply our limit by 4KB if this bit is set.
#[link_section = ".discard"]
const fn pack_gdt(base: u32, limit: u32, perm: u8, s_type: u8, privilege: u8, present: u8, attrs: u8, granularity: u8) -> u64 {
    let mut res: u64 = 0x0;
    res = mask_assign(res, limit as u64, 0, 0, 16);
    res = mask_assign(res, base as u64, 16, 0, 24);
    res = mask_assign(res, perm as u64, 40, 0, 4);
    res = mask_assign(res, s_type as u64, 44, 0, 1);
    res = mask_assign(res, privilege as u64, 45, 0, 2);
    res = mask_assign(res, present as u64, 47, 0, 1);
    res = mask_assign(res, limit as u64, 48, 16, 4);
    res = mask_assign(res, attrs as u64, 52, 0, 3);
    res = mask_assign(res, granularity as u64, 55, 0, 1);
    res = mask_assign(res, base as u64, 56, 24, 8);
    res
}

/// The GDT, Global Descriptor Table.
/// The address of GDT should be 8 byte aligned to get better performance (see *Intel Developer Manual Vol. 3A 3-15*).
#[used]
#[link_section = ".gdt"]
static GDT_TABLE: [u64; GDT_LEN as usize] = [
    // An empty entry (Null Segment) which is reserved by Intel
    pack_gdt(0, 0, 0, 0, 0, 0, 0, 0), 
    // Code Segment, 512KiB
    pack_gdt(0x0, 0x80000, 8, 1, 0, 1, 0b100, 0),
    // Data Segment, 112KiB
    pack_gdt(0x80000, 0x9c000, 3, 1, 0, 1, 0b100, 0),
    // Stack Segment, 112KiB, grow down
    pack_gdt(0x9c000, 0xb8000, 7, 1, 0, 1, 0b100, 0),
    // Video RAM
    pack_gdt(0xb8000, 0xffff, 3, 1, 0, 1, 0b100, 0), 
];

/// An instance of GDT descriptor, occupying 6 bytes in memory.
/// The `limit` field is the length of GDT **in bytes** - 1, which is used by processor 
/// to find the last valid byte in GDT (see *Intel Developer Manual Vol. 3A 3-15*).
#[used]
#[no_mangle]
#[allow(improper_ctypes)]
#[link_section = ".gdt_desc"]
static mut GDT_DESCRIPTOR: GDTDescriptor = GDTDescriptor {
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

    // 4. do a far jump to the next instruction to serialize the processer 
    //   (clear the pipeline, I don't know how does this work =-=)
    asm! {
        "jmp 08h, offset next"
    }

    // 5. jump to code/data segment selector
    asm! {
        "next:",
        "mov eax, 0xdeadbeef"
    }
}

