use core::intrinsics::transmute;
use layout::VIDEO_START;

pub fn display_at(row: u8, col: u8, content: &str) {
    let idx = (row as u32 * 80 + col as u32) * 2;
    for i in 0..content.len() {
        unsafe {
            asm! {
                "mov ah, 0x0c",
                "mov gs:[edi], ax",
                in("edi") idx + i as u32,
                in("al") content.as_bytes()[i]
            }
        }
    }
}

