#[inline(always)]
/// Display a string on screen, see <https://stanislavs.org/helppc/int_10-13.html>.
/// You should only use this function in real mode.
pub fn display_real(src: &str) {
    let ptr = src.as_ptr();
    let len = src.len() as u16;

    unsafe {
        asm! {
            "push bp",
            "mov bp, ax",
            "mov ax, 01301h",
            "mov bx, 000ch",
            "mov dl, 0",
            "int 10h",
            "pop bp",
            in("ax") ptr,
            in("cx") len,
            out("dl") _,
            out("bx") _,
        }
    }
}



