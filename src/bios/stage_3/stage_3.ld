MEMORY {
    real    :   ORIGIN = 0xf000,  LENGTH = 0x1000
    body    :   ORIGIN = 0x10000, LENGTH = 0x6ff00
    ldt     :   ORIGIN = 0x7ff00, LENGTH = 0x100
    data    :   ORIGIN = 0x80000, LENGTH = 0x18000
}

ENTRY(_start);

SECTIONS {
    .real   :   { KEEP(*(.real)) }           > real
    .text   :   { KEEP(*(.startup)) KEEP(*(.stage_3)) *(.text*) }    > body      /* Excutable code */
    .ldt    :   { KEEP(*(.ldt)) }      > ldt       /* LDT */
    .rodata :   { *(.rodata*) }        > data      /* Constants (R/O) */
    .data   :   { *(.data*) }          > data      /* Initialized data */
    .bss    :   { *(.bss*) }           > data      /* Unitnitialized data */
    /DISCARD/ : { *(.eh_frame*) *(.discard)}
}