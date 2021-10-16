use crate::bitwise::mask_assign;
use crate::ring::Privilege;

pub type Selector = u16;
pub type Descriptor = u64;

#[repr()]
pub struct DescriptorTable<const LEN: usize> {
    pub table: &'static mut [u64; LEN],
    pub cur: usize
}

unsafe impl<const LEN: usize> Sync for DescriptorTable<LEN> {}

impl<const LEN: usize> DescriptorTable<LEN> {
    pub fn reset(&mut self) {
        self.table[0..self.cur].fill(0);
        self.cur = 0
    }

    pub fn add(&mut self, entry: u64) -> Result<u16, &'static str> {
        if self.cur >= LEN - 1 {
            return Err("LDT overflow.\n");
        }
        
        self.table[self.cur as usize] = entry;
        self.cur += 1;
        Ok(self.cur as u16 - 1)
    }
}

/// type field enums
pub const SEG_CODE: u8 = 0b1000;
/// Determine whether a data segment expands down.
/// For an expand down segment, the limit field of its GDT entry
/// means the offset can range from limit + 1 to 0xffff/0xffffffff, 
/// which makes it possible to change segment size dynamically 
/// (especially for stack).
pub const SEGD_DOWN: u8 = 0b0100;
pub const SEGD_WRITE: u8 = 0b0010;
/// Mainly for debug usage
pub const SEG_ACCESSED: u8 = 0b0001;

/// This flag determines whether this segment is conforming, 
/// which means whether its allowed for task with lower privilege
/// to jump into this segment. For a conforming code segment, this
/// is allowed, and vice versa.
/// Normally, CPL, which is stored in the lowest 2 bits in CS and SS registers,
/// equals the DPL of the code segment where instructions
/// are being fetched and thus changes correspondingly during task switching. 
/// However, when switching to a conforming code segment, CPL will not
/// be changed (even when DPL < CPL), which means no task switch occurs.
/// Most code segments are nonconforming, i.e. only allow transfer from
/// code segment with the same privilege (without using gates).
/// However, we still need some code segments, for example, math libraries
/// to be conforming to make them accessible for lower privileged code 
/// while prevent them from accessing more privileged data.
pub const SEGC_CONFORM: u8 = 0b0100;
pub const SEGC_READ: u8 = 0b0010;

pub const SEG_LDT: u8 = 2;
/// 32-bit call gate
pub const SEG_CALL_GATE32: u8 = 12;

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
/// | 45:47 | privilege[0:2]  | DPL, 0 = Kernel, 3 = User   |
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
/// For the DPL field: Descriptor privilege level. Determines the 
/// privilege level of this resource.
///  
/// - For a data segment / TSS / Call Gate, only task with 
/// CPL <= DPL (higher privilege) can be allowed to access this resource.
/// - For a nonconforming code segment (without using a call gate), only
/// task with CPL == DPL can access this segment (with selector RPL <= DPL).
/// - For a conforming code segment (or nonconforming code segment accessed 
/// with call gate), only task with CPL >= DPL (lower privilege) can access
/// this segment (selector RPL is not checked).
/// 
/// For the attributes field: 
///
/// - 0: Available to System Programmers flag, reserved
/// - 1: 64-bit code segment
/// - 2: size bit, set if our code is 32-bit, 16-bit vice versa
///
/// For granularity, CPU will multiply our limit by 4KB if this bit is set.
pub const fn pack_seg(base: usize, limit: usize, perm: u8, s_type: u8, privilege: Privilege, present: bool, attrs: u8, granularity: u8) -> Descriptor {
    let mut res: Descriptor = 0x0;
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

/// This function packs a call gate descirptor from given attributes.
/// - seg: the selector of target code segment
/// - entry: the offset of entrypoint in target code segment 
/// - dpl: the privilege needed to invoke this call gate
/// - param_cnt: the count of parameters
/// - valid: whether this call gate is enabled
/// 
/// For more information, see *Intel Developer Manual Vol. 3A 5-13*
pub const fn pack_call_gate(seg: Selector, entry: usize, dpl: Privilege, param_cnt: u8, valid: bool) -> Descriptor {
    let mut res: Descriptor = 0x0;
    res = mask_assign(res, entry as u64, 0, 0, 16);
    res = mask_assign(res, seg as u64, 16, 0, 16);
    res = mask_assign(res, param_cnt as u64, 32, 0, 5);
    res = mask_assign(res, 0 as u64, 37, 0, 3);
    res = mask_assign(res, SEG_CALL_GATE32 as u64, 40, 0, 4);
    res = mask_assign(res, 0 as u64, 44, 0, 1);
    res = mask_assign(res, dpl as u64, 45, 0, 2);
    res = mask_assign(res, valid as u64, 47, 0, 1);
    res = mask_assign(res, entry as u64, 48, 16, 16);
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
/// RPL here is is used for overriding CPL to prevent an privileged application from 
/// requesing restricted resource on behalf a lower privileged application.
/// When requestion resource with a selector, we always compare MAX(CPL, RPL) with
/// DPL to decide whether to grant access.
/// For more information, see *Intel Developer Manual Vol. 3A 3-7 3.4.2 Segment Selectors*
pub const fn pack_selector(index: u16, table: DTType, rpl: Privilege) -> Selector {
    let mut res = 0;
    res = mask_assign(res as u64, rpl as u64, 0, 0, 2) as u16;
    res = mask_assign(res as u64, table as u64, 2, 0, 1) as u16;
    res = mask_assign(res as u64, index as u64, 3, 0, 13) as u16;
    res
}
