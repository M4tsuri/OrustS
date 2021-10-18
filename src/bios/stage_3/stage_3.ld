ENTRY(_start);

SECTIONS {
    .real 61440 : {
        KEEP(*(.real))
    }

    .body 65536 : {  
        KEEP(*(.startup))
        *(.text*)
    }

    .ldt 131072 : {
        KEEP(*(.ldt))
    }
    .tss 135168 : {
        KEEP(*(.tss))
    }

    .data 139264 : {
        *(.rodata*)
        *(.data*)
        *(.bss*)
    }
    /DISCARD/ : { *(.eh_frame*) *(.discard)}
}