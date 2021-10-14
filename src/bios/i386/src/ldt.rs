use layout::LDT_SIZE;

/// LDT is a method of isolating memoyr of tasks from each other.
/// In protect mode, we need at least one GDT, which can be used by all tasks.
/// However, if we want to add more restrictions for a specific task.
/// We need to set a LDT for it and initialize its segment registers with
/// corresponding LDT selectors.
/// LDT can be considered as a sub-GDT, which lays in a seperated segment described by
/// an entry in GDT (a GDT selector). Later when executing lldt instruction with this
/// selector, the address and limit of LDT is loaded in LDTR register, which helps 
/// eliminate address translating when accessing LDT.

use crate::dt_utils::{DTType, pack_selector};
use crate::ring::Privilege;

/// max length of LDT
const LDT_LEN: usize = LDT_SIZE / 8;

#[repr()]
pub struct LDT {
    ldt: &'static mut [u64; LDT_LEN as usize],
    cur: usize
}

unsafe impl Sync for LDT {}

#[used]
#[link_section = ".ldt"]
static mut _LDT_TABLE: [u64; LDT_LEN as usize] = [0; LDT_LEN as usize];

/// The LDT, Local Descriptor Table.
pub static mut LDT_TABLE: LDT = LDT {
    ldt: unsafe { &mut _LDT_TABLE },
    cur: 0
};

impl LDT {
    pub fn reset(&mut self) {
        self.ldt[0..self.cur].fill(0);
        self.cur = 0
    }

    pub fn add(&mut self, entry: u64) -> Result<u16, &'static str> {
        if self.cur >= LDT_LEN - 1 {
            return Err("LDT overflow.\n");
        }
        
        self.ldt[self.cur as usize] = entry;
        self.cur += 1;
        Ok(self.cur as u16 - 1)
    }
}
