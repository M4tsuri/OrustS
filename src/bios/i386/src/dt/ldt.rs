use layout::LDT_SIZE;
use super::utils::DescriptorTable;

/// LDT is a method of isolating memoyr of tasks from each other.
/// In protect mode, we need at least one GDT, which can be used by all tasks.
/// However, if we want to add more restrictions for a specific task.
/// We need to set a LDT for it and initialize its segment registers with
/// corresponding LDT selectors.
/// LDT can be considered as a sub-GDT, which lays in a seperated segment described by
/// an entry in GDT (a GDT selector). Later when executing lldt instruction with this
/// selector, the address and limit of LDT is loaded in LDTR register, which helps 
/// eliminate address translating when accessing LDT.

/// max length of LDT
const LDT_MAX_LEN: usize = LDT_SIZE / 8;

#[used]
#[link_section = ".ldt"]
static mut _LDT_TABLE: [u64; LDT_MAX_LEN] = [0; LDT_MAX_LEN];

/// The LDT, Local Descriptor Table.
pub static mut LDT_TABLE: DescriptorTable<LDT_MAX_LEN> = DescriptorTable {
    table: unsafe { &mut _LDT_TABLE },
    cur: 0
};
