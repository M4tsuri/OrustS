use core::intrinsics::size_of;

/// implements n-bytes padding
pub type Padding<const LEN: usize> = [u8; LEN];

#[allow(non_camel_case_types)]
pub type uint<const LEN: usize> = [u8; LEN];

const fn extend<const SRC: usize, const DEST: usize>(src: &[u8]) -> [u8; DEST] {
    if SRC > DEST {
        panic!("Too wide ux value.")
    }

    let mut tmp: [u8; DEST] = [0_u8; DEST];
    
    let mut i = 0;
    while i < SRC {
        tmp[i] = src[i];
        i += 1;
    }

    tmp
}

pub trait CastUp<T: Sized> {
    fn cast_le(&self) -> T;
    fn cast_be(&self) -> T;
}

macro_rules! impl_cast_up {
    ($t:ty) => {
        impl<const LEN: usize> CastUp<$t> for uint<LEN> {
            fn cast_le(&self) -> $t {
                let tmp = extend::<LEN, {size_of::<$t>()}>(self);
                <$t>::from_le_bytes(tmp)
            }
            
            fn cast_be(&self) -> $t {
                let tmp = extend::<LEN, {size_of::<$t>()}>(self);
                <$t>::from_be_bytes(tmp)
            }
        }
    }
}

impl_cast_up!(u32);
impl_cast_up!(u64);

