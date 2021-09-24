MEMORY {
    body    :   ORIGIN = 0x7e00, LENGTH = 0x71c0
    gdt     :   ORIGIN = 0xefc0, LENGTH = 0x40
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