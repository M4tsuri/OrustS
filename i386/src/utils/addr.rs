pub type Addr32 = u32;
pub type Addr16 = (u16, u16);

/// Convert a 32-bit address to segmented address.
/// For example, address 0x10200 will be converted to (0x1000, 0x0200).
pub const fn to_addr16(addr: Addr32) -> Result<Addr16, &'static str> {
    if addr > (0xffff << 4) + 0xffff {
        Err("Malformed 16 bit addr.\n")
    } else {
        Ok(((addr >> 16) as u16, addr as u16))
    }
}

pub const fn to_addr32(addr: Addr16) -> Addr32 {
    ((addr.0 as u32) << 16) + (addr.1 as u32)
}
