MEMORY {
    body    :   ORIGIN = 0x7c00, LENGTH = 0x7dfe
    magic   :   ORIGIN = 0x7dfe, LENGTH = 0x7e00
}

ENTRY(_start);

SECTIONS {
    .text   :   { *(.startup) *(.text*) }    > body      /* Excutable code */
    .rodata :   { *(.rodata*) } > body      /* Constants (R/O) */
    .data   :   { *(.data*) }    > body      /* Initialized data */
    .bss    :   { *(.bss*) }     > body      /* Unitnitialized data */
    .magic   :   { KEEP(*(.magic)) }     > magic
    /DISCARD/ : { *(.eh_frame*) }
}