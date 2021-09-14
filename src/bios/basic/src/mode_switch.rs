use crate::{mask_assign, gen_mask};

/// the length of GDT, 4 by default (include a null entry)
const GDT_LEN: u8 = 4;

/// A GDT Descriptor descriping the length of GDT and location of GDT in memory.
/// The address of this describtor will be passed to lgdt instruction to fill GDT
#[repr(packed)]
#[allow(improper_ctypes)]
struct GDTDescriptor {
    len: u8,
    addr: *const u64
}

unsafe impl Sync for GDTDescriptor {}

/// Pack data in parameter to a valid GDT entry.
/// A GDT entry can be represented as a 64 bit value, whose fields are defined as follows:
///
/// ```text
/// | 0:16  | limit[0:16]     | segment size limit          |
/// | 16:32 | base[0:16]      | segment base address        |
/// | 32:40 | base[16:24]     |                             |
/// | 40:45 | type[0:5]       | segment type and attributes |
/// | 45:47 | privilege[0:2]  | 0 = Kernel, 3 = User        |
/// | 47:48 | present[0:1]    | 1 = enable segment          |
/// | 48:52 | limit[16:20]    |                             |
/// | 52:55 | attributes[0:3] | segment attributes          |
/// | 55:56 | granularity     | alignment                   |
/// | 56:64 | base[24:32]     |                             |
/// ```
///
/// For the type field, the detailed definition is as follows, see also 
/// http://www.osdever.net/tutorials/view/the-world-of-protected-mode:
///
/// - 0: access flag, set by CPU, we don't care about this
/// - 1: readable flag
/// - 2: conforming flag, less privileged code can jump to this segment if set
/// - 3: code or data flag, set if this is a code or data segment
/// - 4: unknown, seems same as the previous flag? 
///
/// For the attributes field: 
///
/// - 0: Available to System Programmers flag, ignore it
/// - 1: reserved bit, ignore it
/// - 2: size bit, set if out code is 32-bit, 16-bit vice versa
///
/// For granularity, CPU will multiply our limit by 4KB if this bit is set.
#[link_section = ".discard"]
const fn pack_gdt(base: u32, limit: u32, s_type: u8, privilege: u8, present: u8, attrs: u8, granularity: u8) -> u64 {
    let mut res: u64 = 0;
    res = mask_assign!(res, limit as u64, 0, 0, 16);
    res = mask_assign!(res, base as u64, 16, 0, 24);
    res = mask_assign!(res, s_type as u64, 40, 0, 5);
    res = mask_assign!(res, privilege as u64, 45, 0, 2);
    res = mask_assign!(res, present as u64, 47, 0, 1);
    res = mask_assign!(res, limit as u64, 48, 16, 4);
    res = mask_assign!(res, attrs as u64, 52, 0, 3);
    res = mask_assign!(res, granularity as u64, 55, 0, 1);
    res = mask_assign!(res, base as u64, 56, 24, 8);
    res
}

/// The GDT, occupying 32 bytes in memory
#[used]
#[link_section = ".gdt"]
static GDT_TABLE: [u64; GDT_LEN as usize] = [
    // An empty entry (Null Segment) which is reserved by Intel
    pack_gdt(0, 0, 0, 0, 0, 0, 0), 
    pack_gdt(0x40000, 0xfffff, 0, 0, 0, 0, 0),
    0xcc, 
    0xcc
];

/// An instance of GDT descriptor, occupying 5 bytes in memory
#[used]
#[no_mangle]
#[link_section = ".gdt"]
static mut GDT_DESCRIPTOR: GDTDescriptor = GDTDescriptor {
    len: GDT_LEN,
    addr: GDT_TABLE.as_ptr()
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
#[inline]
pub unsafe fn to_protect() {
    extern "C" {
        static GDT_DESCRIPTOR: GDTDescriptor;
    }

    // 1. Disable maskable hardware interrupts
    asm!("cli");
    
    // 2. Execute `lgdt` instruction to load address of GDT to GDTR register.
    //    Here we directly use a externed symbol in instruction, so linker will help 
    //    us relocate it to its real address at compile time
    asm!("lgdt GDT_DESCRIPTOR");

    // 3. Set PE flag in control register CR0, which activates segmentation.
    //    If needed, set PG flag for paging.
    asm! {
        "mov eax, cr0",
        "or eax, 1",
        "mov cr0, eax"
    }


}

