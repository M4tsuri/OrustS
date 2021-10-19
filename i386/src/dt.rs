
pub mod packers;
pub mod gdt;
pub mod consts;

/// Descriptor table type, GDT or LDT
#[repr(u8)]
pub enum DTType {
    GDT = 0,
    LDT = 1
}

pub type Selector = u16;
pub type Descriptor = u64;

pub struct DescriptorTable<const LEN: usize> {
    pub table: &'static mut [Descriptor; LEN],
    pub cur: usize
}

unsafe impl<const LEN: usize> Sync for DescriptorTable<LEN> {}

impl<const LEN: usize> DescriptorTable<LEN> {
    pub fn reset(&mut self) {
        self.table[0..self.cur].fill(0);
        self.cur = 0
    }

    /// Replace the original table with a new one
    pub fn replace(&mut self, src: &[Descriptor]) -> Result<(), &'static str> {
        self.reset();
        for entry in src {
            self.add(*entry)?;
        }
        Ok(())
    }

    /// Add an entry to this descriptor table
    pub fn add(&mut self, entry: Descriptor) -> Result<u16, &'static str> {
        if self.cur >= LEN - 1 {
            return Err("[add] Descriptor Table Overflow.\n");
        }
        
        self.table[self.cur as usize] = entry;
        self.cur += 1;
        Ok(self.cur as u16 - 1)
    }
}
