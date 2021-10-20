/// disable NMI (Non-maskable hardware interrupts)
pub fn cli() {
    unsafe {
        asm!("cli");
    }
}
