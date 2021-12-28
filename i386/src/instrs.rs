use core::arch::asm;

// FIXME: encapsulate operations on control registers

/// protect mode
pub const CR0_PE: u32 = 1 << 0;
pub const CR0_MP: u32 = 1 << 1;
pub const CR0_EM: u32 = 1 << 2;
/// task switched
pub const CR0_TS: u32 = 1 << 3;
pub const CR0_ET: u32 = 1 << 4;
pub const CR0_NE: u32 = 1 << 5;
pub const CR0_WP: u32 = 1 << 16;
pub const CR0_AM: u32 = 1 << 18;
/// Not-write through
pub const CR0_NW: u32 = 1 << 29;
/// cache disable
pub const CR0_CD: u32 = 1 << 30;
/// paging
pub const CR0_PG: u32 = 1 << 31;

pub const CR4_VME: u32 = 1 << 0;
pub const CR4_PVI: u32 = 1 << 1;
pub const CR4_TSD: u32 = 1 << 2;
pub const CR4_DE: u32 = 1 << 3;
/// page size is 4KiB if unset, and 4MiB when set.
/// This bit is ignored in PAE mode or x86-64 long mode.
pub const CR4_PSE: u32 = 1 << 4;
/// physical address extension
pub const CR4_PAE: u32 = 1 << 5;
pub const CR4_MCE: u32 = 1 << 6;
pub const CR4_PGE: u32 = 1 << 7;
pub const CR4_PCE: u32 = 1 << 8;
pub const CR4_OSF: u32 = 1 << 9;
pub const CR4_OSXMMEXCPT: u32 = 1 << 10;
pub const CR4_UMIP: u32 = 1 << 11;
pub const CR4_LA57: u32 = 1 << 12;
pub const CR4_VMXE: u32 = 1 << 13;
pub const CR4_SMXE: u32 = 1 << 14;
pub const CR4_FSGSBASE: u32 = 1 << 16;
pub const CR4_PCIDE: u32 = 1 << 17;
pub const CR4_OSXSAVE: u32 = 1 << 18;
pub const CR4_SMEP: u32 = 1 << 20;
pub const CR4_SMAP: u32 = 1 << 21;
pub const CR4_PKE: u32 = 1 << 22;
pub const CR4_CET: u32 = 1 << 23;
pub const CR4_PKS: u32 = 1 << 24;

/// disable NMI (Non-maskable hardware interrupts)
#[inline(always)]
pub fn cli() {
    unsafe {
        asm!("cli");
    }
}

#[inline(always)]
pub fn inb(port: u16) -> u8 {
    let data: u8;
    unsafe {
        asm!("in al, dx", in("dx") port, out("al") data)
    }
    data
}

#[inline(always)]
pub fn inw(port: u16) -> u16 {
    let data: u16;
    unsafe {
        asm!("in ax, dx", in("dx") port, out("ax") data)
    }
    data
}

#[inline(always)]
pub fn indw(port: u16) -> u32 {
    let data: u32;
    unsafe {
        asm!("in eax, dx", in("dx") port, out("eax") data)
    }
    data
}

#[inline(always)]
pub fn outb(port: u16, data: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") data)
    }
}

#[inline(always)]
pub fn outw(port: u16, data: u16) {
    unsafe {
        asm!("out dx, ax", in("dx") port, in("ax") data)
    }
}

#[inline(always)]
pub fn outdw(port: u16, data: u32) {
    unsafe {
        asm!("out dx, eax", in("dx") port, in("eax") data)
    }
}
