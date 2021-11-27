/// Provide some functions for displaying content on screen

pub mod s80x25c16;

pub enum VideoError {
    BufferOverflow,
}

#[derive(Clone, Copy)]
pub struct Cursor(usize, usize);

pub struct Screen<'a, T> 
where
    T: VideoBuf + ?Sized
{
    /// The cursor always points to the location after the previous writing
    cursor: Cursor,
    buf: &'a mut T
}

impl<'a, T: VideoBuf + Sized> Iterator for Screen<'a, T> {
    type Item = Cursor;

    fn next(&mut self) -> Option<Self::Item> {
        let (height, width) = self.buf.get_shape();
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

impl<'a, T: VideoBuf + Sized> Screen<'a, T> {
    pub fn new(buf: &'a mut T) -> Self {
        Self {
            cursor: Cursor(0, 0),
            buf
        }
    }

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
    /// return the length of displayed string on ok
    fn putc(&mut self, src: u8) -> Result<usize, VideoError> {
        self.print_raw(&[src])
    }

    fn print(&mut self, src: &str) -> Result<usize, VideoError> {
        self.print_raw(src.as_bytes())
    }

    fn println(&mut self, src: &str) -> Result<usize, VideoError> {
        self.print(src)?;
        self.putc(b'\n')
    }

    fn print_raw(&mut self, src: &[u8]) -> Result<usize, VideoError>;
}

impl<'a, T: VideoBuf + Sized> Printable for Screen<'a, T> {
    fn print_raw(&mut self, src: &[u8]) -> Result<usize, VideoError> {
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
        
        Ok(src.len())
    }
}
