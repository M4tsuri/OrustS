use core::mem::size_of;

use crate::addr::to_addr16;

/// The type of a memory range, returned by e820 syscall
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
#[repr(packed)]
pub struct E820MemRange {
    base: u64,
    len: u64,
    /// type of this memory range
    ty: E820MemType
}

pub fn get_mem_info(buf: &mut [E820MemRange]) -> Result<u32, &'static str> {
    let buf_addr = to_addr16(buf.as_ptr() as u32)?;
    let mut range_num: u32 = 0;
    let mut is_failed: u16 = 0;
    unsafe {
        asm! {
            // its garanteed that es is always 0 in real mode
            "mov es, dx",
            "int 0x15",
            "xor dx, dx",
            "mov es, dx",
            "mov ax, 1",
            "cmovc dx, ax",
            inout("eax") 0xe820_u32 => _,
            inout("di") buf_addr.1 => _,
            inout("dx") buf_addr.0 => is_failed,
            inout("ecx") buf.len() * size_of::<E820MemRange>() => range_num,
        }
    }

    if is_failed == 1 {
        Err("Error reading memory info")   
    } else {
        Ok(range_num)
    }
}
