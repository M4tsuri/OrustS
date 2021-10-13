use crate::bitwise::mask_assign;
use crate::ring::Privilege;

/// A GDT Descriptor descriping the length of GDT and location of GDT in memory.
/// The address of this describtor will be passed to lgdt instruction to fill GDT.
///
/// The `limit` field is the length of GDT **in bytes** - 1, which is used by processor 
/// to find the last valid byte in GDT (see *Intel Developer Manual Vol. 3A 3-15*).
#[repr(packed)]
#[allow(improper_ctypes)]
pub struct GDTDescriptor {
    pub limit: u16,
    pub base_address: *const u64
}

unsafe impl Sync for GDTDescriptor {}

/// type field enums
pub const SEG_CODE: u8 = 0b1000;
pub const SEGD_DOWN: u8 = 0b0100;
pub const SEGD_WRITE: u8 = 0b0010;
pub const SEG_ACCESSED: u8 = 0b0001;

pub const SEGC_CONFORM: u8 = 0b0100;
pub const SEGC_READ: u8 = 0b0010;

pub const SEG_LDT: u8 = 2;

/// S flags
/// system management segment 
pub const TYPE_SYS: u8 = 0;
/// code or data segment
pub const TYPE_CD: u8 = 1;

/// available for system software use
pub const ATTR_AVL: u8 = 0b001;
/// 64bit code segment (IA-32e only)
pub const ATTR_CODE64: u8 = 0b010;
/// 32-bit segment
pub const ATTR_SEG32: u8 = 0b100;
/// 16-bit segment
pub const ATTR_SEG16: u8 = 0b000;

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
/// For the type field, see *Intel Developer Manual 3-12 Vol.3A Table 3-1*:
/// | 11 | 10 |  9 |  8 |
/// |  D |  E |  W |  A |
/// |  C |  C |  R |  A |
///
/// For the s field: clear if this is a system segment, set if this is a code/data segment 
///
/// For the attributes field: 
///
/// - 0: Available to System Programmers flag, reserved
/// - 1: 64-bit code segment
/// - 2: size bit, set if our code is 32-bit, 16-bit vice versa
///
/// For granularity, CPU will multiply our limit by 4KB if this bit is set.
pub const fn pack_dt(base: usize, limit: usize, perm: u8, s_type: u8, privilege: u8, present: u8, attrs: u8, granularity: u8) -> u64 {
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

/// Descriptor table type, GDT or LDT
#[repr(u8)]
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
    res = mask_assign(res as u64, index as u64, 3, 0, 13) as u16;
    res
}
