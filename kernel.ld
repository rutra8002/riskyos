OUTPUT_ARCH(riscv)
ENTRY(_start)

SECTIONS {
    . = 0x80200000; /* ram address */

    .text : {
        *(.text.boot)   /* start.S goes first */
        *(.text*) /*rest like rust etc*/
    }

    .rodata : { *(.rodata*) } /*readonly data*/

    .data : { *(.data*) } /*otherdata, static variables etc*/
    /*set part of ram which is needed to be zeored*/
    .bss : {
        __bss_start = .;
        *(.bss*)
        *(COMMON)
        __bss_end = .;
    }

    /* 64 KiB stack */
    . = ALIGN(16);
    . += 0x10000;
    __stack_top = .;
}