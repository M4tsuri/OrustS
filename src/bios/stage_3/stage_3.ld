ENTRY(_start);

SECTIONS {
    .body 65536 : {  
        KEEP(*(.startup)) 
        KEEP(*(.real))
        KEEP(*(.stage_3))
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