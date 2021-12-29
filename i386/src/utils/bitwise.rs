//! Some bitwise operations which may be useful for writing out code.
//! Functions and macros in this module must not appear in the object file.
//! Since macros are expanded at compile-time, we do not need to worry about them.
//! But functions must be const, and explictly declared to be put in .discard section, 
//! which will be dropped during linking.

/// Assign src[src_start:src_start + len] bit to dest[dest_start:dest_start + len]
/// TODO: for some naive but universal circumstances, for example, len = 1 or src_start = 0, 
/// consider using faster algorithm. This can be done by replacing mask_assign function with
/// a macro
pub const fn mask_assign(mut dest: u64, src: u64, dest_start: u8, src_start: u8, len: u8) -> u64 {
    let dest_mask = gen_mask(dest_start, len);
    let src_mask = gen_mask(src_start, len);

    // clear corresponding bits in dest 
    dest &= !dest_mask;

    let gap: i8 = (src_start as i8 - dest_start as i8).abs();

    dest | if dest_start >= src_start {
        (src & src_mask) << gap
    } else {
        (src & src_mask) >> gap
    }
}

pub const fn gen_mask(start: u8, len: u8) -> u64 {
    if start + len >= 64 {
        !0 ^ ((1 << start) - 1)
    } else {
        ((1 << (start + len)) - 1) ^ ((1 << start) - 1)
    }   
}
