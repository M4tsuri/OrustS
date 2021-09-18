MEMORY {
    body    :   ORIGIN = 0x7c00, LENGTH = 0x1fe
    /* the address of gdt should be 8 byte aligned to get better performance, see Intel Developer Manual Vol. 3A 3-15 */
    magic   :   ORIGIN = 0x7dfe, LENGTH = 0x2
    stage2  :   ORIGIN = 0x7e00, LENGTH = 0x0
}

ENTRY(_start);

SECTIONS {
    .text   :   { *(.startup) *(.stage_1) *(.text*)}    > body      /* Excutable code */
    .rodata :   { *(.rodata*) }        > body      /* Constants (R/O) */
    .data   :   { *(.data*) }          > body      /* Initialized data */
    .bss    :   { *(.bss*) }           > body      /* Unitnitialized data */
    /* keep the order of GDT and the GDT Descriptor to make sure that base address of GDT meets the alignment requirement (see https://users.informatik.haw-hamburg.de/~krabat/FH-Labor/gnupro/5_GNUPro_Utilities/c_Using_LD/ldLinker_scripts.html#Input_section_basics) */
    .magic  :   { KEEP(*(.magic)) }    > magic
    .stage2 :   { KEEP(*(.stage_2)) }        > stage2
    /DISCARD/ : { *(.eh_frame*) *(.discard)}
}