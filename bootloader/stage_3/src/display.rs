use i386::screen::{Cursor, Printable, Screen, s80x25c16::{Buffer, WIDTH, HEIGHT}};

#[link_section = ".video"]
static mut VIDEO_BUFFER: Buffer = [[0; WIDTH]; HEIGHT];

/// FIXME: consider using lazy_static with a mutex here
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
