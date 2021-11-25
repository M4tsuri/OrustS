/// disable NMI (Non-maskable hardware interrupts)
#[inline(always)]
pub fn cli() {
    unsafe {
        asm!("cli");
    }
}

/// The pause instruction is for telling processor that we are 
/// in a spin-wait loop to make some optimization.
#[inline(always)]
pub fn pause() {
    unsafe {
        asm!("pause")
    }
}

#[inline(always)]
pub fn inb(port: u16) -> u8 {
    let data: u8;
    unsafe {
        asm!("in dx, al", in("dx") port, out("al") data)
    }
    data
}

#[inline(always)]
pub fn inw(port: u16) -> u16 {
    let data: u16;
    unsafe {
        asm!("in dx, ax", in("dx") port, out("ax") data)
    }
    data
}

#[inline(always)]
pub fn indw(port: u16) -> u32 {
    let data: u32;
    unsafe {
        asm!("in dx, eax", in("dx") port, out("eax") data)
    }
    data
}

#[inline(always)]
pub fn outb(port: u16, data: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") data)
    }
}

#[inline(always)]
pub fn outw(port: u16, data: u16) {
    unsafe {
        asm!("out dx, ax", in("dx") port, in("ax") data)
    }
}

#[inline(always)]
pub fn outdw(port: u16, data: u32) {
    unsafe {
        asm!("out dx, eax", in("dx") port, in("eax") data)
    }
}
