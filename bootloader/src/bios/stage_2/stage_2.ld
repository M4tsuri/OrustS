ENTRY(_start);

SECTIONS {
    .body 32256 : {
        *(.startup) 
        *(.stage_2) 
        *(.text*)
        *(.rodata*)
        *(.data*)
        *(.bss*)
    }      /* Excutable code */
    .gdt 65280 : {
        KEEP(*(.gdt)) 
    }
    .gdt_desc  : {
        KEEP(*(.gdt_desc))
    }
    /* keep the order of GDT and the GDT Descriptor to make sure that base address of GDT meets the alignment requirement (see https://users.informatik.haw-hamburg.de/~krabat/FH-Labor/gnupro/5_GNUPro_Utilities/c_Using_LD/ldLinker_scripts.html#Input_section_basics) */
    .stage3 65536 (NOLOAD) : { 
        KEEP(*(.stage_3)) 
    }
    /DISCARD/ : { *(.eh_frame*) *(.discard)}
}