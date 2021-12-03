use core::mem::size_of;

use crate::utils::addr::to_addr16;

/// The type of a memory range, returned by e820 syscall
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum E820MemType {
    /// This run is available RAM usable by the operating system.
    AddressRangeMemory = 1,
    /// This run of addresses is in use or reserved 
    /// by the system, and must not be used by the operating system.
    AddressRangeReserved = 2,
    Undefined,
}

/// The returned structure of E820 bios function
#[derive(Clone, Copy)]
#[repr(packed)]
pub struct E820MemRange {
    pub base: u64,
    pub len: u64,
    /// type of this memory range
    pub ty: E820MemType
}

#[derive(Clone, Copy)]
pub struct E820MemInfo<const MAX: usize> {
    pub len: usize,
    pub ranges: [E820MemRange; MAX]
}

unsafe impl<const MAX: usize> Sync for E820MemInfo<MAX> {}

impl<const MAX: usize> E820MemInfo<MAX> {
    pub const fn new() -> Self {
        Self {
            len: 0,
            ranges: [E820MemRange {
                base: 0,
                len: 0,
                ty: E820MemType::Undefined
            }; MAX]
        }
    }
    /// read memory information with e820 interrupt. if the lenge of array ranges
    /// is very small, the result may be unsound.
    pub fn query(&mut self) -> Option<()> {
        self.len = get_mem_info(&mut self.ranges)?;
        Some(())
    }

    /// get ranges as a slice, returns None if an error occurred 
    /// (theoritically impossible)
    pub fn get_ranges<'a>(&'a self) -> Option<&'a [E820MemRange]> {
        self.ranges.get(..self.len)
    }
}

/// return the number of read ranges on success, None on failure
fn get_mem_info(buf: &mut [E820MemRange]) -> Option<usize> {
    let buf_addr = to_addr16(buf.as_ptr() as u32)?;
    let mut range_num: usize;
    let mut is_failed: u16;
    unsafe {
        asm! {
            // its garanteed that es is always 0 in real mode
            "push ebx",
            "xor ebx, ebx",
            "mov es, dx",
            "mov edx, 0x534D4150",
            "int 0x15",
            "xor dx, dx",
            "mov es, dx",
            "mov ax, 1",
            "cmovc dx, ax",
            "pop ebx",
            inout("eax") 0xe820_u32 => _,
            inout("di") buf_addr.1 => _,
            inout("dx") buf_addr.0 => is_failed,
            inout("ecx") buf.len() * size_of::<E820MemRange>() => range_num,
        }
    }

    if is_failed == 1 {
        None
    } else {
        Some(range_num / size_of::<E820MemRange>())
    }
}
