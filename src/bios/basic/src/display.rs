#[link_section = ".stage1"]
/// display a string on screen, see https://stanislavs.org/helppc/int_10-13.html
pub fn display(src: &str) {
    let ptr = src.as_ptr();
    let len = src.len() as u16;

    unsafe {
        asm! {
            "mov ax, {0:x}",
            "mov bp, ax",
            "mov cx, {1:x}",
            "mov ax, 01301h",
            "mov bx, 000ch",
            "mov dl, 0",
            "int 10h",
            in(reg) ptr,
            in(reg) len
        }
    }
}



