pub mod nofs;

pub enum FSError<E> {
    UnknownError,
    NoEnoughSpace,
    FileNotFound,
    NotImplemented,
    DiskError(E)
}

impl<T> From<T> for FSError<T> {
    fn from(e: T) -> Self {
        Self::DiskError(e)
    }
}

/// A file system should have these traits.
/// P is an identifier for a file/directory
pub trait FileSystem<I, E> {
    fn alloc(&mut self, size: usize) -> Result<I, FSError<E>>;
    fn extend(&mut self, orig: I, new_size: usize) -> Result<I, FSError<E>>;
    fn delete(&mut self, id: I) -> Result<(), FSError<E>>;
    fn write(&mut self, id: I, src: &[u8]) -> Result<usize, FSError<E>>;
    fn read(&self, id: I, dest: &mut [u8]) -> Result<usize, FSError<E>>;
}

