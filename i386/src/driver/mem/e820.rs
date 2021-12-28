use core::{
    mem::size_of, 
    ops::ControlFlow,
    arch::asm
};
use crate::{
    utils::addr::to_addr16,
    mem::PhysAddr
};

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
#[repr(C, align(8))]
pub struct E820MemRange {
    pub base: PhysAddr,
    pub len: PhysAddr,
    /// type of this memory range
    pub ty: E820MemType
}

impl Into<&'static str> for E820MemType {
    fn into(self) -> &'static str {
        match self {
            E820MemType::AddressRangeMemory => "Memory",
            E820MemType::AddressRangeReserved => "Reserved",
            E820MemType::Undefined => "Undefined",
        }
    }
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
    let mut range_num = 0;
    assert_eq!(size_of::<E820MemRange>(), 24);

    match buf.iter_mut().try_fold(0, |bx, ebuf| {
        let next: u16;
        let magic: u32;
        let is_failed: u16;
        let buf_addr = match to_addr16(ebuf as *const E820MemRange as u32) {
            Some(addr) => addr,
            None => return ControlFlow::Break(-1)
        };
        unsafe {
            asm! {
                // its garanteed that es is always 0 in real mode
                "push ebx",
                "mov bx, {SEQ:x}",
                "mov es, dx",
                "mov edx, 0x534D4150",
                "int 0x15",
                "mov {SEQ:x}, bx",
                "xor dx, dx",
                "mov es, dx",
                "setc dl",
                "pop ebx",
                SEQ = inout(reg) bx => next,
                inout("eax") 0xe820_u32 => magic,
                inout("di") buf_addr.1 => _,
                inout("dx") buf_addr.0 => is_failed,
                inout("ecx") size_of::<E820MemRange>() => _,
            }
        }

        if magic != 0x534D4150 || is_failed == 1 {
            return ControlFlow::Break(-1)
        }
        
        if next == 0 {
            ControlFlow::Break(0)
        } else {
            range_num += 1;
            ControlFlow::Continue(next)
        }
    }) {
        ControlFlow::Break(0) | ControlFlow::Continue(_) => Some(range_num),
        ControlFlow::Break(_) => None
    }
}
