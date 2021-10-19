/// Consts and magic numbers in Descriptors.

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
/// 32 bit avaliable tss
pub const SEG_AVAIL_TSS32: u8 = 9;
/// task gate
pub const SEG_TASK_GATE: u8 = 5;

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