use i386::gdt::GDTSelector;

/// This module defined functons and data structure for returning to real mode.
/// For details of the switching, see 
/// *Intel Developer Manual 9-14 Vol. 3A 9.9.2 Switching Back to Real-Address Mode*


/// Switch to real mode. 
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

#[no_mangle]
#[link_section = ".real"]
unsafe fn _to_real() {
    // 4. Load segment registers
    asm! {
        ".code16",
        "mov dx, ax",
        "mov ax, {normal}",
        "mov ss, ax",
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        normal = const GDTSelector::NORMAL as u8
    }

    // 5. clear cr0.PE to enter real address mode
    asm! {
        "mov eax, cr0",
        "and eax, 0xfffe",
        "mov cr0, eax"
    }

    // 6. Execute a far jump to a real address mode program. This also sets the cs register
    asm! {
    // 7. load segment registers
        "xor ax, ax",
        "mov ss, ax",
        "mov ds, ax",
        "mov fs, ax",
        "mov es, ax",
        "mov sp, 0x7b00",
        "push 0",
        "push dx",
        "retf"
    }
}

#[link_section = ".real"]
pub unsafe fn poweroff() {
    asm! {
        ".code16",
        "mov ax, 0x5307",
        "mov bx, 0x0001",
        "mov cx, 0x0003",
        "int 15h"
    }
}
