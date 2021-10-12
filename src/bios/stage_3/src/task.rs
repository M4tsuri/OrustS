use i386::ring::Privilege;
use i386::addr::{to_addr16, to_addr32};
use i386::ldt::LDT_TABLE;
use i386::dt_utils::pack_dt;
use i386::gdt::GDTSelector;

/// Represent a task.
/// Load and run a task with this struct.
pub struct Task {
    /// privilege of the task
    privilege: Privilege,
    /// offset of this task in memory, currently a task has only a code segment
    offset: (u16, u16),
    size: u32,
    code_selector: u16
}

impl Task {
    pub fn init_ldt(&mut self) -> Result<(), &'static str> {
        let seg = self.offset.0;
        let code_desc = pack_dt(to_addr32((seg, 0)), self.size - 1, 8, 1, 
            Privilege::Ring0 as u8, 1, 0b100, 0);
        self.code_selector = unsafe { LDT_TABLE.add(code_desc, Privilege::Ring0)? };
        Ok(())
    }

    pub fn transfer(&self) {
        unsafe {
            asm! {
                "lldt {0:x}",
                "push {1:x}",
                "push {2:x}",
                "retf",
                in(reg) GDTSelector::LDT as u16,
                in(reg) self.code_selector,
                in(reg) self.offset.1
            }
        }
    }

    /// Init a task from disk, user need to specify drive index, task offset and task size
    pub fn new(privilege: Privilege, offset: u32, size: u32) -> Result<Self, &'static str> {
        Ok(Self {
            privilege,
            offset: to_addr16(offset)?,
            size,
            code_selector: 0
        })
    }
}

