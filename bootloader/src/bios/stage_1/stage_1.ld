ENTRY(_start);

SECTIONS {
    .body 31744 : { 
        *(.startup)
        *(.text*)
        *(.rodata*)
        *(.data*)
        *(.bss*)
    }   /* Excutable code */
    /* keep the order of GDT and the GDT Descriptor to make sure that base address of GDT meets the alignment requirement (see https://users.informatik.haw-hamburg.de/~krabat/FH-Labor/gnupro/5_GNUPro_Utilities/c_Using_LD/ldLinker_scripts.html#Input_section_basics) */
    .magic 0x7dfe : { 
        SHORT(0xaa55)
    }
    .stage2 32256 (NOLOAD) : { 
        *(.stage_2) 
    }
    /DISCARD/ : { *(.eh_frame*) *(.discard)}
}