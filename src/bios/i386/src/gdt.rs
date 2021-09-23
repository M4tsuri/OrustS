use crate::dt_utils::{DTType, GDTDescriptor, pack_gdt, pack_selector};
use crate::ring::Privilege;
use layout::*;

/// The length of GDT, 6 by default (include a null entry).
/// Current max length is 0x40 / 8 = 8, which is specified in linker script in stage 2
const GDT_LEN: u16 = 6;

/// The GDT, Global Descriptor Table.
/// The address of GDT should be 8 byte aligned to get better performance (see *Intel Developer Manual Vol. 3A 3-15*).
#[used]
#[link_section = ".gdt"]
static GDT_TABLE: [u64; GDT_LEN as usize] = [
    // An empty entry (Null Segment) which is reserved by Intel
    pack_gdt(0, 0, 0, 0, 
        Privilege::Ring0 as u8, 0, 0, 0), 
    // Code Segment, 512KiB
    pack_gdt(CODE_START, CODE_END, 8, 1, 
        Privilege::Ring0 as u8, 1, 0b100, 0),
    // Data Segment, 112KiB
    pack_gdt(DATA_START, DATA_END, 3, 1, 
        Privilege::Ring0 as u8, 1, 0b100, 0),
    // Stack Segment, 112KiB, grow down
    pack_gdt(STACK_START, STACK_END, 7, 1, 
        Privilege::Ring0 as u8, 1, 0b110, 0),
    // Video RAM
    pack_gdt(0xb8000, 0xffff, 3, 1, 
        Privilege::Ring0 as u8, 1, 0b100, 0), 
    pack_gdt(0, 0x0ffff, 2, 1, 
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
    NORMAL = pack_selector(5, DTType::GDT, Privilege::Ring0)
}
