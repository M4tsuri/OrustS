MEMORY {
    rom (rwx) : ORIGIN = 31744, LENGTH = 32256 - 31744
    stage2_ptr (!r!w!x) : ORIGIN = 32256, LENGTH = 0
}

ENTRY(_start);

SECTIONS {
    .body : { 
        *(.startup)
        *(.text*)
        *(.rodata*)
        *(.data*)
        *(.bss*)
    }   /* Excutable code */ > rom
    /* keep the order of GDT and the GDT Descriptor to make sure that base address of GDT meets the alignment requirement (see https://users.informatik.haw-hamburg.de/~krabat/FH-Labor/gnupro/5_GNUPro_Utilities/c_Using_LD/ldLinker_scripts.html#Input_section_basics) */
    .magic 0x7dfe : { 
        SHORT(0xaa55)
    } > rom

    .stage2  (NOLOAD) : { 
        *(.stage_2) 
    } > stage2_ptr
    /DISCARD/ : { *(.eh_frame*) *(.discard)}
}