use core::mem::size_of;

/// This module is only intended to be used by bootloader to setup a initial GDT
use i386::mem::dt::{packers::*, consts::*};
use i386::mem::dt::{DescriptorTable, Descriptor, DTType};
use crate::layout::*;
use i386::ring::Privilege;

/// The length of GDT, 8 by default (include a null entry).
/// Current max length is 0x100 / 8 = 32, which is specified in linker script in stage 2
const GDT_MAX_LEN: usize = GDT_SIZE as usize / size_of::<Descriptor>();

/// number of GDT entries which are hard-coded.
const GDT_RESERVED_LEN: usize = 8;

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
    /// The DPL and RPL of a stack segment descriptor and selector must be
    /// the same with the CPL when loading to ss register.
    STACK = pack_selector(3, DTType::GDT, Privilege::Ring0),
}

const fn init_gdt() -> [u64; GDT_MAX_LEN] {
    let mut gdt: [u64; GDT_MAX_LEN] = [0; GDT_MAX_LEN];
    // An empty entry (Null Segment) which is reserved by Intel
    gdt[0] = pack_desc(0, 0, 
        0, 0, 
        Privilege::Ring0, false, 0, 0);
    // Code Segment, 4G
    gdt[1] = pack_desc(0, CODE_END - 1, 
        SEG_CODE | SEGC_READ, TYPE_CD, 
        Privilege::Ring0, true, ATTR_SEG32, 0);
    // Data Segment, 4G
    gdt[2] = pack_desc(0, DATA_END - 1, 
        SEGD_WRITE, TYPE_CD, 
        Privilege::Ring0, true, ATTR_SEG32, 0);
    // Stack Segment, unlimited
    gdt[3] = pack_desc(0, 0, 
        SEGD_DOWN | SEGD_WRITE, TYPE_CD,
        Privilege::Ring0, true, ATTR_SEG32, 0);
    gdt
}

/// The GDT, Global Descriptor Table.
/// The address of GDT should be 8 byte aligned to get better performance (see *Intel Developer Manual Vol. 3A 3-15*).
#[used]
#[link_section = ".gdt"]
static mut _GDT_TABLE: [Descriptor; GDT_MAX_LEN] = init_gdt();

/// A wrapper for GDT, we use this wrapper to dynamically change the content of
/// GDT, thus gives us more extensibility.
pub static mut GDT_TABLE: DescriptorTable<GDT_MAX_LEN> = DescriptorTable {
    table: unsafe { &mut _GDT_TABLE },
    cur: GDT_RESERVED_LEN
};
