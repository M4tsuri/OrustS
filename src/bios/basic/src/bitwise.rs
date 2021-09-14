/// Some bitwise operations which may be useful for writing out code.

#[macro_export]
macro_rules! mask_assign {
    ($dest:expr, $src:expr, $dest_start:expr, $src_start:expr, $len:expr) => {{
        let dest_mask = gen_mask!($dest_start, $len);
        let src_mask = gen_mask!($src_start, $len);

        // clear corresponding bits in dest 
        $dest &= !dest_mask;

        $dest | if $dest_start > $src_start {
            ($src & src_mask) << ($dest_start - $src_start)
        } else {
            ($src & src_mask) << ($src_start - $dest_start)
        }
    }}
}

#[macro_export]
macro_rules! gen_mask {
    ($start:expr, $len:expr) => {{
        (1 << ($start + $len) - 1) ^ (1 << $start - 1)
    }}
}

