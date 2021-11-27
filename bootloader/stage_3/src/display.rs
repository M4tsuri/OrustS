use core::{intrinsics::transmute, slice::from_raw_parts_mut};

use i386::screen::{Cursor, Printable, Screen, s80x25c16::{Buffer, WIDTH, HEIGHT}};
use shared::layout::VIDEO_START;

#[link_section = ".video"]
static mut VIDEO_BUFFER: Buffer = [[0; WIDTH]; HEIGHT];

pub static mut SCREEN: Screen<Buffer> = Screen {
    cursor: Cursor(0, 0),
    buf: unsafe { &mut VIDEO_BUFFER }
};

pub fn scr_clear() {
    unsafe { SCREEN.clear() }
}

pub fn print(s: &str) {
    unsafe { SCREEN.print(s); }
}
