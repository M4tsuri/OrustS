MEMORY {
    body    :   ORIGIN = 0x7e00, LENGTH = 0x78200
}

ENTRY(_start);

SECTIONS {
    .text   :   { *(.startup) *(.stage_2) }    > body      /* Excutable code */
    .rodata :   { *(.rodata*) }        > body      /* Constants (R/O) */
    .data   :   { *(.data*) }          > body      /* Initialized data */
    .bss    :   { *(.bss*) }           > body      /* Unitnitialized data */
    /DISCARD/ : { *(.eh_frame*) *(.discard)}
}