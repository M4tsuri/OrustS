use shared::gdt::GDTSelector;

/// This module defined functons and data structure for returning to real mode.
/// For details of the switching, see 
/// *Intel Developer Manual 9-14 Vol. 3A 9.9.2 Switching Back to Real-Address Mode*

/// Switch to real mode. 
#[inline]
pub fn to_real(target_offset: u16) {
    unsafe {
        // TODO 1. Disable interrupts. Due to a strange bug, we did not enable it after entering
        //    protect mode.
        // asm!("cli")

        // TODO 2. if paging is enabled...

        // 3. Transfer the program control to a readable segment that has a limit of 64 KBytes
        asm! {
            "jmp {CS}, offset {real}",
            CS = const GDTSelector::SWITCH as u8,
            real = sym _to_real,
            in("ax") target_offset,
        }
    }
}

/// In fact, aftering entering protect mode, there is no such an option for 
/// turning it off. However, we can pretend to turning it off by assign each 
/// segment with ring 0 privilege. Note that after doing this, only privilege 
/// checking is disabled, limit checking and type checking are still carried out.
/// See *Intel Developer Manual Vol.3A 5-1*
#[no_mangle]
#[link_section = ".real"]
unsafe fn _to_real() {
    asm! {
        ".code16",
        "movzx edx, ax",
        // 4. Load segment registers
        "mov ax, {normal}",
        "mov ss, ax",
        "mov ds, ax",
        "mov fs, ax",
        "mov es, ax",
        "mov gs, ax",
        // 5. clear cr0.PE to enter real address mode
        "mov eax, cr0",
        "and eax, 0xfffe",
        "mov cr0, eax",
        // 6. load segment registers
        "xor ax, ax",
        "mov ss, ax",
        "mov ds, ax",
        "mov fs, ax",
        "mov es, ax",
        "mov gs, ax",
        "mov sp, 0x7b00",
        // 7. Execute a far jump to a real address mode program. This also sets the cs register
        "push 0x00000000",
        "push edx",
        "retf",
        ".code32",
        normal = const GDTSelector::NORMAL as u8,
        out("edx") _,
        out("eax") _
    }
}

#[link_section = ".real"]
pub unsafe fn poweroff() {
    asm! {
        ".code16",
        "int 15h",
        ".code32",
        in("ax") 0x5307,
        in("bx") 0x0001,
        in("cx") 0x0003
    }
}
