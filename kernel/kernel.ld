ENTRY(main);

SECTIONS {
    .text 0x100000 : { 
        *(.text*)
    }   /* Excutable code */
    .data : {
        *(.rodata*)
        *(.data*)
        *(.bss*)
    }
    /DISCARD/ : { *(.eh_frame*)}
}