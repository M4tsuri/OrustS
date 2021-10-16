use super::utils::{ATTR_SEG16, ATTR_SEG32, DTType, DescriptorTable, SEGC_READ, SEGD_DOWN, SEGD_WRITE, SEG_CODE, SEG_LDT, TYPE_CD, TYPE_SYS, pack_seg, pack_selector};
use crate::ring::Privilege;
use layout::*;

/// A GDT Descriptor descriping the length of GDT and location of GDT in memory.
/// The address of this describtor will be passed to lgdt instruction to fill GDT.
///
/// The `limit` field is the length of GDT **in bytes** - 1, which is used by processor 
/// to find the last valid byte in GDT (see *Intel Developer Manual Vol. 3A 3-15*).
#[repr(packed)]
#[allow(improper_ctypes)]
pub struct GDTDescriptor {
    pub limit: u16,
    pub base_address: &'static [u64; GDT_MAX_LEN]
}

unsafe impl Sync for GDTDescriptor {}

/// The length of GDT, 8 by default (include a null entry).
/// Current max length is 0x100 / 8 = 32, which is specified in linker script in stage 2
const GDT_MAX_LEN: usize = GDT_SIZE as usize / 8;

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
    VIDEO = pack_selector(4, DTType::GDT, Privilege::Ring0),
    SWITCH = pack_selector(5, DTType::GDT, Privilege::Ring0),
    NORMAL = pack_selector(6, DTType::GDT, Privilege::Ring0),
    LDT = pack_selector(7, DTType::GDT, Privilege::Ring0)
}

const fn init_gdt() -> [u64; GDT_MAX_LEN] {
    let mut gdt: [u64; GDT_MAX_LEN] = [0; GDT_MAX_LEN];
    // An empty entry (Null Segment) which is reserved by Intel
    gdt[0] = pack_seg(0, 0, 
        0, 0, 
        Privilege::Ring0 as u8, 0, 0, 0);
    // Code Segment, 512KiB, code execute and read
    gdt[1] = pack_seg(0, CODE_END - 1, 
        SEG_CODE | SEGC_READ, TYPE_CD, 
        Privilege::Ring0 as u8, 1, ATTR_SEG32, 0);
    // Data Segment, 112KiB, data Read/Write,accessed
    gdt[2] = pack_seg(0, DATA_END - 1, 
        SEGD_WRITE, TYPE_CD, 
        Privilege::Ring0 as u8, 1, ATTR_SEG32, 0);
    // Stack Segment, 112KiB, grow down, note for a grow down segment, 
    // available offset ranges from limit + 1 to 0xffffffff (or 0xffff)
    // so decrease limit allocates new memory for this segment.
    gdt[3] = pack_seg(0, STACK_START, 
        SEGD_DOWN | SEGD_WRITE, TYPE_CD,
        Privilege::Ring0 as u8, 1, ATTR_SEG32, 0);
    // Video RAM
    gdt[4] = pack_seg(VIDEO_START, VIDEO_SIZE - 1, 
        SEGD_WRITE, TYPE_CD, 
        Privilege::Ring0 as u8, 1, ATTR_SEG32, 0);
    // A normal segment for executing code to switch to real mode in protect mode.
    // We make a far jump to code in this segment in protect mode to load cs register
    // with a segment descriptor with suitable limit and other attributes.
    // To prevent errors, this segment should satisfy the following conditions:
    // 1. A 16-bit code segment to make sure our processor works correctly after entering real mode.
    // 2. A small segment with limit of 0FFFFh
    //    i.e. max limit is 0FFFFh to meet real mode addressing limitations
    // 3. Start at 0 to make logical address and linear address consistent.
    gdt[5] = pack_seg(0, NORMAL_END - 1, 
        SEGC_READ | SEG_CODE, TYPE_CD, 
        Privilege::Ring0 as u8, 1, ATTR_SEG16, 0);
    // A normal segment for mode switching, this is a 16 bit writable data segment.
    // This segment overlaps with the previous one to meet the real mode unsegmented model.
    // The descriptor of this segment will be loaded to ss, es, fs, gs, ds after entering real mode.
    gdt[6] = pack_seg(0, NORMAL_END - 1, 
        SEGD_WRITE, TYPE_CD, 
        Privilege::Ring0 as u8, 1, ATTR_SEG16, 0);
    // Segment for LDT, note this descriptor is system management descriptor,
    // so s_type bit is clear.
    // See *Intel Developer Manual 3-14 Vol. 3A* for perm field definitions.
    gdt[7] = pack_seg(LDT_START, LDT_SIZE - 1, 
        SEG_LDT, TYPE_SYS, 
        Privilege::Ring0 as u8, 1, 0b000, 0);
    gdt
}

/// The GDT, Global Descriptor Table.
/// The address of GDT should be 8 byte aligned to get better performance (see *Intel Developer Manual Vol. 3A 3-15*).
#[used]
#[link_section = ".gdt"]
static mut _GDT_TABLE: [u64; GDT_MAX_LEN] = init_gdt();

/// An instance of GDT descriptor, occupying 6 bytes in memory.
/// The `limit` field is the length of GDT **in bytes** - 1, which is used by processor 
/// to find the last valid byte in GDT (see *Intel Developer Manual Vol. 3A 3-15*).
#[used]
#[no_mangle]
#[allow(improper_ctypes)]
#[link_section = ".gdt_desc"]
pub static mut GDT_DESCRIPTOR: GDTDescriptor = GDTDescriptor {
    limit: GDT_RESERVED_LEN as u16 * 8 - 1,
    base_address: unsafe { &_GDT_TABLE }
};

impl GDTDescriptor {
    /// Update the gdt descriptor and then update gdtr.
    /// This function should be called in a task with CPL of ring 0.
    pub fn update(&mut self, src: &'static DescriptorTable<GDT_MAX_LEN>) {
        self.limit = src.cur as u16 * 8 - 1;
        self.base_address = src.table;
        unsafe {
            asm!("lgdt {}", sym GDT_DESCRIPTOR)
        }
    }
}

/// A wrapper for GDT, we use this wrapper to dynamically change the content of
/// GDT, thus gives us more extensibility.
pub static mut GDT_TABLE: DescriptorTable<GDT_MAX_LEN> = DescriptorTable {
    table: unsafe { &mut _GDT_TABLE },
    cur: GDT_RESERVED_LEN
};
