use crate::bitwise::mask_assign;
use crate::ring::Privilege;
use super::consts::*;
use super::{Descriptor, Selector, DTType};

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
pub const fn pack_desc(base: usize, limit: usize, perm: u8, s_type: u8, privilege: Privilege, present: bool, attrs: u8, granularity: u8) -> Descriptor {
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

/// Pack a TSS descriptor.
/// - base: base address of the segment with tss in it
/// - limit: max allowed offset in the segment
/// - dpl: the least privilege (CPL) required for dispatching the task indeciated by this tss
/// - present: whether this segment is enabled
/// - busy: set if the corresponding task is running / suspended
/// - avl: available for system software use, never mind
/// - granulatity: multiple limit by 4k if set
/// 
/// **Be aware that TSS descriptor can only be located in GDT**
/// For more information, see *Intel Developer Manual Vol. 3A 7-6*
pub const fn pack_tss_desc(base: usize, limit: usize, dpl: Privilege, present: bool, busy: bool, avl: bool, granularity: u8) -> Descriptor {
    let perm = SEG_AVAIL_TSS32 | ((busy as u8) << 1);
    let attrs = 0b000 | (avl as u8);
    pack_desc(base, limit, perm, TYPE_SYS, dpl, present, attrs, granularity)
}

/// Pack a task gate descriptor.
/// Task gates provide indirect access to TSS descriptor from GDT / LDT / IDT.
/// 
/// - tss_desc: the target tss descriptor in GDT
/// - dpl: the lowest privilege required to access this task gate. Note that when 
/// DPL of task gate is used for privilege validating, the DPL of target TSS descriptor
/// is not used.
/// - present: whether this descriptor is enabled.
/// 
/// We need task gates for the following needs:
/// - Since the busy flag only locates at TSS descriptor, we cannot have multiple 
/// descriptors for one task. But we can use several task gates to reference one task.
/// - Task Gate can reside in GDT/LDT/IDT and it can override the DPL of tss descriptor,
/// so a task with insufficient privilege to directly access the TSS descriptor can 
/// access a specific task through a task gate.
/// - Task can be used for interrupt or exception handling (when residing in IDT). 
pub const fn pack_task_gate(tss_desc: u16, dpl: Privilege, present: bool) -> Descriptor {
    pack_desc(tss_desc as usize, 0, SEG_TASK_GATE, TYPE_SYS, dpl, present, 0, 0)
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


