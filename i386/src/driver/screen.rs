//! Provide some functions for displaying content on screen

pub mod s80x25c16;

pub enum VideoError {
    BufferOverflow,
}

#[derive(Clone, Copy)]
pub struct Cursor(pub usize, pub usize);

pub struct Screen<'a, T> 
where
    T: VideoBuf
{
    /// The cursor always points to the location after the previous writing
    pub cursor: Cursor,
    pub buf: &'a mut T
}

impl<'a, T: VideoBuf> Iterator for Screen<'a, T> {
    type Item = Cursor;

    fn next(&mut self) -> Option<Self::Item> {
        let (_, width) = self.buf.get_shape();
        let cursor = self.cursor;
        // check if we need wrap line
        if self.cursor.1 == width - 1 {
            self.newline();
        } else {
            self.cursor.1 += 1
        }

        Some(cursor)
    }
}

impl<'a, T: VideoBuf> Screen<'a, T> {
    /// The cursor will not move if its already at the last line.
    /// Return true if a cursor movement occurs
    pub fn cursor_down(&mut self) -> bool {
        let (height, _) = self.buf.get_shape();
        // wrap line, now check if we need scroll
        if self.cursor.0 == height - 1 {
            false
        } else {
            self.cursor.0 += 1;
            true
        }
    }

    pub fn newline(&mut self) {
        if !self.cursor_down() {
            // scroll up by 1 line
            self.buf.up();
        }
        self.cursor.1 = 0;
    }

    pub fn set_cursor(&mut self, row: usize, col: usize) {
        self.cursor.0 = row;
        self.cursor.1 = col;
    }

    pub fn clear(&mut self) {
        self.buf.clear();
        self.set_cursor(0, 0);
    }
}

pub trait VideoBuf {
    type Item;
    /// Note that row number comes
    fn get_shape(&self) -> (usize, usize);
    fn clear(&mut self);
    fn up(&mut self);
    fn down(&mut self);
    /// get the byte sequence to write to video buffer for the specified char
    fn get_charseq(&self, ch: u8) -> Self::Item;
    /// get the location of cursor
    fn set_at(&mut self, cur: Cursor, data: Self::Item);
}

pub trait Printable {
    fn putc(&mut self, src: u8) {
        self.print_raw(&[src])
    }

    fn print(&mut self, src: &str) {
        self.print_raw(src.as_bytes())
    }

    fn print_raw(&mut self, src: &[u8]);
}

impl<'a, T: VideoBuf> core::fmt::Write for Screen<'a, T> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print_raw(s.as_bytes());
        Ok(())
    }
}

impl<'a, T: VideoBuf> Printable for Screen<'a, T> {
    fn print_raw(&mut self, src: &[u8]) {
        for &ch in src {
            // FIXME: we should deal with newline in print function 
            if ch == b'\n' {
                self.newline();
                continue;
            }
            let item = self.buf.get_charseq(ch);
            let cur = self.next().unwrap();
            self.buf.set_at(cur, item);
        }
    }
}
