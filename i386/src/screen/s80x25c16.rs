use super::VideoBuf;

pub const HEIGHT: usize = 25;
pub const WIDTH: usize = 80;

pub type Buffer<'a> = &'a mut [[u16; WIDTH]; HEIGHT];

impl<'a> VideoBuf for Buffer<'a> {
    type Item = u16;

    fn get_shape(&self) -> (usize, usize) {
        (HEIGHT, WIDTH)
    }

    fn clear(&mut self) {
        self.fill([0; WIDTH]);
    }

    fn up(&mut self) {
        for i in 0..HEIGHT - 2 {
            self[i] = self[i + 1]
        }
        self[HEIGHT - 1].fill(0);
    }

    fn down(&mut self) {
        for i in 2..HEIGHT - 1 {
            self[i] = self[i - 1]
        }
        self[0].fill(0);
    }

    fn get_charseq(&self, ch: u8) -> Self::Item {
        (0x0c << 8) | ch as u16
    }

    fn set_at(&mut self, cur: super::Cursor, data: Self::Item) {
        self[cur.0][cur.1] = data
    }
}
