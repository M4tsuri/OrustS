use super::{DTError, Descriptor, DescriptorTable};
use core::arch::asm;

/// A GDT Descriptor descriping the length of GDT and location of GDT in memory.
/// The address of this describtor will be passed to lgdt instruction to fill GDT.
///
/// The `limit` field is the length of GDT **in bytes** - 1, which is used by processor 
/// to find the last valid byte in GDT (see *Intel Developer Manual Vol. 3A 3-15*).
#[repr(packed)]
#[allow(improper_ctypes)]
pub struct GDTDescriptor<'a, const LEN: usize> {
    pub limit: u16,
    pub base_address: &'a [Descriptor; LEN]
}

unsafe impl<'a, const LEN: usize> Sync for GDTDescriptor<'a, LEN> {}

impl<'a, const LEN: usize> GDTDescriptor<'a, LEN> {
    /// Update the gdt descriptor and then update gdtr.
    /// This function should be called in a task with CPL of ring 0.
    pub fn update(src: &'a DescriptorTable<LEN>) -> Result<(), DTError> {
        if src.cur == 0 {
           return Err(DTError::EmptyTable)
        }
        // this is enforced by intel
        if src.table[0] != 0 {
            return Err(DTError::ErrorReservedEntry)
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

