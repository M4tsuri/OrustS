ENTRY(_start);

SECTIONS {
    .real  : {
        KEEP(*(.real))
    }

    .body 65536 : {  
        KEEP(*(.startup))
        *(.text*)
    }

    .ldt  : {
        KEEP(*(.ldt))
    }
    .tss  : {
        KEEP(*(.tss))
    }

    .data 139264 : {
        *(.rodata*)
        *(.data*)
        *(.bss*)
    }
    /DISCARD/ : { *(.eh_frame*) *(.discard)}
}