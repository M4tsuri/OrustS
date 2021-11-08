MEMORY {
    code (rx) : ORIGIN = 65536, LENGTH = 139264 - 65536
    data (rw) : ORIGIN = 139264, LENGTH = 196608 - 139264
}

ENTRY(_start);

SECTIONS {
    .body : {  
        KEEP(*(.startup))
        *(.text*)
        FILL(0)
        . = ORIGIN(code) + LENGTH(code);
    } > code

    .data : {
        *(.rodata*)
        *(.data*)
        *(.bss*)
    } > data
    .fill : {
        FILL(0)
        . = ORIGIN(data) + LENGTH(data) - 1;
        BYTE(0)
    }

    /DISCARD/ : { *(.eh_frame*) *(.discard)}
}