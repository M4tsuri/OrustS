ENTRY(_start);

SECTIONS {
    .video 753664 (NOLOAD) : {
        *(.video)
    }

    .body 65536 : {  
        KEEP(*(.startup))
        *(.text*)
        FILL(0)
        . = 139264;
    }

    .data 139264 : {
        *(.rodata*)
        *(.data*)
        *(.bss*)
    }

    .fill : {
        FILL(0)
        . = 327680 - 1;
        BYTE(0)
    }
    /DISCARD/ : { *(.eh_frame*) *(.discard)}
}