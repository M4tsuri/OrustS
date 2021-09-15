use crate::bitwise::mask_assign;
use crate::ring::Privilege;

/// the length of GDT, 3 by default (include a null entry)
const GDT_LEN: u16 = 5;

/// A GDT Descriptor descriping the length of GDT and location of GDT in memory.
/// The address of this describtor will be passed to lgdt instruction to fill GDT.
///
/// The `limit` field is the length of GDT **in bytes** - 1, which is used by processor 
/// to find the last valid byte in GDT (see *Intel Developer Manual Vol. 3A 3-15*).
#[repr(packed)]
#[allow(improper_ctypes)]
pub struct GDTDescriptor {
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

/// The GDT selector encodes 
///
/// - index of the entry on a GDT for a segment
/// - type (GDT or LDT) of a segment
/// - privilege level of a segment level, note that this indicates the 
///   requested privilege level (RPL). CPL is the privilege level of current task.
///   When a task is asking access for a segment with a selector with lower privilege 
///   than CPL, the request will not be permitted, and vice versa. 
///   (See *Intel Developer Manual Vol. 3A 5-6 5.5 PRIVILEGE LEVELS*)
///
/// into a 16-bit integer. After entering protected mode, segment registers should hold 
/// values of selectors.
#[repr(u16)]
pub enum GDTSelector {
    NULL = pack_selector(0, DTType::GDT, Privilege::Ring0),
    CODE = pack_selector(1, DTType::GDT, Privilege::Ring0),
    DATA = pack_selector(2, DTType::GDT, Privilege::Ring0),
    VIDEO = pack_selector(3, DTType::GDT, Privilege::Ring0)
}

/// Descriptor table type, GDT or LDT
pub enum DTType {
    GDT = 0,
    LDT = 1
}

/// Pack attributes of a selector into the hardcoded selector.
/// Note that **index is the entry index in 8-byte array**.
/// For more information, see *Intel Developer Manual Vol. 3A 3-7 3.4.2 Segment Selectors*
#[link_section = ".discard"]
pub const fn pack_selector(index: u16, table: DTType, rpl: Privilege) -> u16 {
    let mut res = 0;
    res = mask_assign(res as u64, rpl as u64, 0, 0, 2) as u16;
    res = mask_assign(res as u64, table as u64, 2, 0, 1) as u16;
    res = mask_assign(res as u64, (index * 8) as u64, 3, 0, 13) as u16;
    res
}
