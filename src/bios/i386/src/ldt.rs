use crate::dt_utils::{DTType, pack_selector};
use crate::ring::Privilege;

/// Current max length is 0x100 / 8 = 32, which is specified in linker script in stage 2
const LDT_LEN: u16 = 31;

#[repr()]
pub struct LDT {
    ldt: &'static mut [u64; LDT_LEN as usize],
    cur: u16
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
        self.ldt.fill(0);
        self.cur = 0
    }

    pub fn add(&mut self, entry: u64, ring: Privilege) -> Result<u16, &'static str> {
        if self.cur >= LDT_LEN - 1 {
            return Err("LDT overflow.\n");
        }
        
        self.ldt[self.cur as usize] = entry;
        self.cur += 1;
        Ok(pack_selector(self.cur - 1, DTType::LDT, ring))
    }
}
