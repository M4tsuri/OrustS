/// disable NMI (Non-maskable hardware interrupts)
#[inline(always)]
pub fn cli() {
    unsafe {
        asm!("cli");
    }
}


