ENTRY(_start);

SECTIONS {
    .real 61440 : {
        KEEP(*(.real))
    }

    .body 65536 : {  
        KEEP(*(.startup))
        *(.text*)
    }

    .ldt 491520 : {
        KEEP(*(.ldt))
    }
    .tss 495616 : {
        KEEP(*(.tss))
    }

    .data 499712 : {
        *(.rodata*)
        *(.data*)
        *(.bss*)
    }
    /DISCARD/ : { *(.eh_frame*) *(.discard)}
}