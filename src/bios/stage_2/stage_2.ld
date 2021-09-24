MEMORY {
    body    :   ORIGIN = 0x7e00, LENGTH = 0x7100
    gdt     :   ORIGIN = 0xef00, LENGTH = 0x100
    /**
     * Note that stage 3 actually starts at 0xf000, but the entrypoint of 
     * stage 3 locates at 0x10000, so we just skip 0x1000 here. 
     */
    stage3  :   ORIGIN = 0x10000, LENGTH = 0x0
}

ENTRY(_start);

SECTIONS {
    .text   :   { *(.startup) *(.stage_2) *(.text*)}    > body      /* Excutable code */
    .rodata :   { *(.rodata*) }        > body      /* Constants (R/O) */
    .data   :   { *(.data*) }          > body      /* Initialized data */
    .bss    :   { *(.bss*) }           > body      /* Unitnitialized data */
    .gdt    :   { KEEP(*(.gdt)) KEEP(*(.gdt_desc))}      > gdt
    /* keep the order of GDT and the GDT Descriptor to make sure that base address of GDT meets the alignment requirement (see https://users.informatik.haw-hamburg.de/~krabat/FH-Labor/gnupro/5_GNUPro_Utilities/c_Using_LD/ldLinker_scripts.html#Input_section_basics) */
    .stage3 :   { KEEP(*(.stage_3)) }        > stage3
    /DISCARD/ : { *(.eh_frame*) *(.discard)}
}