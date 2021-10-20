use super::{Descriptor, DescriptorTable};

/// A GDT Descriptor descriping the length of GDT and location of GDT in memory.
/// The address of this describtor will be passed to lgdt instruction to fill GDT.
///
/// The `limit` field is the length of GDT **in bytes** - 1, which is used by processor 
/// to find the last valid byte in GDT (see *Intel Developer Manual Vol. 3A 3-15*).
#[repr(packed)]
#[allow(improper_ctypes)]
pub struct GDTDescriptor<const LEN: usize> {
    pub limit: u16,
    pub base_address: &'static [Descriptor; LEN]
}

unsafe impl<const LEN: usize> Sync for GDTDescriptor<LEN> {}

impl<const LEN: usize> GDTDescriptor<LEN> {
    /// Update the gdt descriptor and then update gdtr.
    /// This function should be called in a task with CPL of ring 0.
    pub fn update(src: &'static DescriptorTable<LEN>) -> Result<(), &'static str> {
        if src.cur == 0 {
           return Err("Empty GDT.\n")
        }
        // this is enforced by intel
        if src.table[0] != 0 {
            return Err("GDT[0] not empty.\n")
        }

        let desc = Self {
            limit: src.cur as u16 * 8 - 1,
            base_address: src.table
        };
        
        unsafe {
            asm!("lgdt [{:e}]", in(reg) &desc)
        }
        Ok(())
    }
}

