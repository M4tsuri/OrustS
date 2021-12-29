use core::fmt::{Arguments, Write};
use i386::driver::screen::{
    Cursor, 
    Screen, 
    s80x25c16::{Buffer, WIDTH, HEIGHT}
};

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

pub fn _print(s: Arguments) -> core::fmt::Result {
    unsafe { SCREEN.write_fmt(s) }
}

/// print with format string 
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::display::_print(format_args!($($arg)*)).unwrap());
}

/// print with format string with newline
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}
