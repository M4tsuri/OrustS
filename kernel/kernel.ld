ENTRY(main);

SECTIONS {
    . = 0x100000;
    .text ALIGN(4K) : { 
        *(.startup)
        *(.text*)
    }   /* Excutable code */
    .data ALIGN(4K) : {
        *(.rodata*)
        *(.data*)
        *(.bss*)
    }
    /DISCARD/ : { *(.eh_frame*)}
}