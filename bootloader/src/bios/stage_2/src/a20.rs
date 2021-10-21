use core::ptr::addr_of;

/// Code for checking and enabling the A20 line
pub fn check_a20() -> bool {
    let mut magic: usize = 0xdeadbeef;
    let magic_ptr = addr_of!(magic) as usize;

    unsafe { 
        asm! {
            "xor edx, edx",
            "mov dx, es",
            "push dx",
            "mov dx, ss",
            "lea edx, [edx + 0xf800]",
            "mov es, dx",
            "lea edx, [{magic_ptr} + 0x8000]",
            "mov dword ptr es:[edx], 0xbabecafe",
            "pop dx",
            "mov es, dx",
            "mov edx, dword ptr ss:[{magic_ptr}]",
            magic_ptr = in(reg) magic_ptr & 0xffff,
            out("edx") magic
        }
    }

    return !(magic == 0xbabecafe)
}

