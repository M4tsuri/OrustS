/// Under 80x25 16 color mode, each char on screen is defined by a 2 byte value.
/// The higher byte defines the background and text color.
/// While the lower byte defined what the charactor is.
pub fn display_at(row: u8, col: u8, content: &str) {
    let idx = (row as u32 * 80 + col as u32) * 2;
    for i in 0..content.len() {
        unsafe {
            asm! {
                "mov gs:[edi], ax",
                in("edi") idx + (i * 2) as u32,
                in("ax") (0x0c << 8) | (content.as_bytes()[i] as u16)
            }
        }
    }
}

