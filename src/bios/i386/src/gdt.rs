use crate::dt_utils::{DTType, GDTDescriptor, SEGC_READ, SEGD_DOWN, SEGD_WRITE, SEG_CODE, SEG_LDT, TYPE_CD, TYPE_SYS, pack_dt, pack_selector};
use crate::ring::Privilege;
use layout::*;

/// The length of GDT, 8 by default (include a null entry).
/// Current max length is 0x100 / 8 = 32, which is specified in linker script in stage 2
const GDT_LEN: u16 = 8;

/// The GDT, Global Descriptor Table.
/// The address of GDT should be 8 byte aligned to get better performance (see *Intel Developer Manual Vol. 3A 3-15*).
#[used]
#[link_section = ".gdt"]
static GDT_TABLE: [u64; GDT_LEN as usize] = [
    // An empty entry (Null Segment) which is reserved by Intel
    pack_dt(0, 0, 0, 0, 
        Privilege::Ring0 as u8, 0, 0, 0), 
    // Code Segment, 512KiB, code execute-only
    pack_dt(0, CODE_END - 1, SEG_CODE | SEGC_READ, TYPE_CD, 
        Privilege::Ring0 as u8, 1, 0b100, 0),
    // Data Segment, 112KiB, data Read/Write,accessed
    pack_dt(0, DATA_END - 1, SEGD_WRITE, TYPE_CD, 
        Privilege::Ring0 as u8, 1, 0b100, 0),
    // Stack Segment, 112KiB, grow down, note for a grow down segment, 
    // available offset ranges from limit + 1 to 0xffffffff (or 0xffff)
    // so decrease limit allocates new memory for this segment.
    pack_dt(0, STACK_START, SEGD_DOWN | SEGD_WRITE, TYPE_CD,
        Privilege::Ring0 as u8, 1, 0b100, 0),
    // Video RAM
    pack_dt(VIDEO_START, VIDEO_SIZE - 1, SEGD_WRITE, TYPE_CD, 
        Privilege::Ring0 as u8, 1, 0b100, 0), 
    // A normal segment for executing code to switch to real mode in protect mode.
    // We make a far jump to code in this segment in protect mode to load cs register
    // with a segment descriptor with suitable limit and other attributes.
    // To prevent errors, this segment should satisfy the following conditions:
    // 1. A 16-bit code segment to make sure our processor works correctly after entering real mode.
    // 2. A small segment with limit of 0FFFFh
    //    i.e. max limit is 0FFFFh to meet real mode addressing limitations
    // 3. Start at 0 to make logical address and linear address consistent.
    pack_dt(0, NORMAL_END - 1, SEGC_READ | SEG_CODE, TYPE_CD, 
        Privilege::Ring0 as u8, 1, 0b000, 0),
    // A normal segment for mode switching, this is a 16 bit writable data segment.
    // This segment overlaps with the previous one to meet the real mode unsegmented model.
    // The descriptor of this segment will be loaded to ss, es, fs, gs, ds after entering real mode.
    pack_dt(0, NORMAL_END - 1, SEGD_WRITE, TYPE_CD, 
        Privilege::Ring0 as u8, 1, 0b000, 0),
    // Segment for LDT, note this descriptor is system management descriptor,
    // so s_type bit is clear.
    // See *Intel Developer Manual 3-14 Vol. 3A* for perm field definitions.
    pack_dt(LDT_START, LDT_SIZE - 1, SEG_LDT, TYPE_SYS, 
        Privilege::Ring0 as u8, 1, 0b000, 0)
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
    STACK = pack_selector(3, DTType::GDT, Privilege::Ring0),
    VIDEO = pack_selector(4, DTType::GDT, Privilege::Ring0),
    SWITCH = pack_selector(5, DTType::GDT, Privilege::Ring0),
    NORMAL = pack_selector(6, DTType::GDT, Privilege::Ring0),
    LDT = pack_selector(7, DTType::GDT, Privilege::Ring0)
}
