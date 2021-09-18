MEMORY {
    body    :   ORIGIN = 0x10000, LENGTH = 0x70000
    data    :   ORIGIN = 0x80000, LENGTH = 0x1c000
}

ENTRY(_start);

SECTIONS {
    .text   :   { *(.startup) *(.stage_2) }    > body      /* Excutable code */
    .rodata :   { *(.rodata*) }        > data      /* Constants (R/O) */
    .data   :   { *(.data*) }          > data      /* Initialized data */
    .bss    :   { *(.bss*) }           > data      /* Unitnitialized data */
    /DISCARD/ : { *(.eh_frame*) *(.discard)}
}