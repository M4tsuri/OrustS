use i386::ring::Privilege;
use shared::ldt::LDT_TABLE;
use i386::dt::packers::{pack_desc, pack_selector};
use i386::dt::DTType;
use shared::gdt::GDTSelector;

/// Represent a task.
/// Load and run a task with this struct.
pub struct Task {
    /// privilege of the task
    privilege: Privilege,
    /// offset of this task in memory, currently a task has only a code segment
    offset: usize,
    size: usize,
    code_selector: u16
}

impl Task {
    /// In this function, we dynamically add new entry to LDT and get the corresponding
    /// selector.
    /// Currently this functions is only a Proof-of-Concept, 
    /// maybe we won't use is afterwards.
    /// **Note we did not reset the LDT.**
    pub fn init_ldt(&mut self) -> Result<(), &'static str> {
        let code_desc = pack_desc(self.offset, self.size - 1, 8, 1, 
            Privilege::Ring0, true, 0b100, 0);
        let idx = unsafe { LDT_TABLE.add(code_desc)? };
        self.code_selector = pack_selector(idx, DTType::LDT, Privilege::Ring0);
        Ok(())
    }

    /// Transfer control to the target task, currently is only a demo.
    /// TODO: transfer control with the help of TSS
    pub fn transfer(&self) {
        unsafe {
            asm! {
                "lldt {0:x}",
                "push {1:e}",
                "push {2:e}",
                "retf",
                in(reg) GDTSelector::LDT as u16,
                in(reg) self.code_selector,
                in(reg) 0
            }
        }
    }

    /// Create a new object representing a task.
    pub fn new(privilege: Privilege, offset: usize, size: usize) -> Self {
        Self {
            privilege,
            offset,
            size,
            code_selector: 0
        }
    }
}
