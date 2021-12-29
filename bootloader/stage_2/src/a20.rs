use core::{
    ptr::addr_of,
    arch::asm
};


/// Code for checking and enabling the A20 line.
/// If the A20 line of a CPU is not enabled, the max address it can use is 0xfffff
/// due to hardware limitation. All memory access with address larger than it will be 
/// wrapped around.
/// For example, the address 0xf800:0x8000 will be wrapped around to 0x0.
/// So we can check whether A20 line is enabled with this interesting mechanism.
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

pub fn enable_a20() {
    unsafe {
        asm! {
            "in al, 0x92",
            "or al, 2",
            "out 0x92, al",
            out("al") _
        }
    }
}

