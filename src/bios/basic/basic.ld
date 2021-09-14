MEMORY {
    body    :   ORIGIN = 0x7c00, LENGTH = 0x1be
    gdt     :   ORIGIN = 0x7dbe, LENGTH = 0x40
    magic   :   ORIGIN = 0x7dfe, LENGTH = 0x2
}

ENTRY(_start);

SECTIONS {
    .text   :   { *(.startup) *(.text*) *(.stage1) }    > body      /* Excutable code */
    .rodata :   { *(.rodata*) }        > body      /* Constants (R/O) */
    .data   :   { *(.data*) }          > body      /* Initialized data */
    .bss    :   { *(.bss*) }           > body      /* Unitnitialized data */
    .gdt    :   { KEEP(*(.gdt)) }      > gdt
    .magic  :   { KEEP(*(.magic)) }    > magic
    /DISCARD/ : { *(.eh_frame*) *(.discard)}
}