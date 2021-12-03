ENTRY(_start);

SECTIONS {
    .meminfo 65024 (NOLOAD) : {
        KEEP(*(.meminfo))
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

    .video 753664 (NOLOAD) : {
        *(.video)
    }

    .stage3 1048576 (NOLOAD) : { 
        KEEP(*(.kernel)) 
    }
    /DISCARD/ : { *(.eh_frame*) *(.discard)}
}