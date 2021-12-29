use core::{
    fmt::{Arguments, Write}, 
    intrinsics::transmute
};
use i386::driver::screen::{Cursor, Screen, s80x25c16::Buffer};
use shared::layout::VIDEO_START;
use spin::Mutex;

lazy_static! {
    pub static ref SCREEN: Mutex<Screen<'static, Buffer>> = Mutex::new(Screen {
        cursor: Cursor(0, 0),
        buf: unsafe {
            transmute::<usize, &mut Buffer>(VIDEO_START)
        }
    });
}

pub fn scr_clear() {
    SCREEN.lock().clear()
}

pub fn _print(s: Arguments) -> core::fmt::Result {
    SCREEN.lock().write_fmt(s)
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
