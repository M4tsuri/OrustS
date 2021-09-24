use crate::dt_utils::{DTType, GDTDescriptor, pack_dt, pack_selector};
use crate::ring::Privilege;
use layout::*;

/// Current max length is 0x100 / 8 = 32, which is specified in linker script in stage 2
const LDT_LEN: u16 = 3;

/// The LDT, Local Descriptor Table.
#[used]
#[link_section = ".ldt"]
static mut LDT_TABLE: [u64; LDT_LEN as usize] = [0, 0, 0];

/// The DT selector encodes 
///
/// - index of the entry on a DT for a segment
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
pub enum LDTSelector {
    CODE = pack_selector(1, DTType::GDT, Privilege::Ring0),
    DATA = pack_selector(2, DTType::GDT, Privilege::Ring0),
    STACK = pack_selector(3, DTType::GDT, Privilege::Ring0)
}
