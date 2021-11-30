MEMORY {
    rom (rwx) : ORIGIN = 32256, LENGTH = 65536 - 32256
    stage3_ptr (!r!w!x) : ORIGIN = 65536, LENGTH = 0
}

ENTRY(_start);

SECTIONS {
    .body 32256 : {
        *(.startup)
        *(.text*)
        *(.rodata*)
        *(.data*)
        *(.bss*)
    }      /* Excutable code */ > rom
    .meminfo 65024 : {
        KEEP(*(.meminfo))
    } > rom
    .gdt 65280 : {
        KEEP(*(.gdt)) 
        FILL(0)
        . = ORIGIN(rom) + LENGTH(rom);
    } > rom

    /* keep the order of GDT and the GDT Descriptor to make sure that base address of GDT meets the alignment requirement (see https://users.informatik.haw-hamburg.de/~krabat/FH-Labor/gnupro/5_GNUPro_Utilities/c_Using_LD/ldLinker_scripts.html#Input_section_basics) */
    .stage3 65536 (NOLOAD) : { 
        KEEP(*(.stage_3)) 
    } > stage3_ptr
    /DISCARD/ : { *(.eh_frame*) *(.discard)}
}