.section .text.boot
.global _start

_start:
    csrr  t0, mhartid #which core am i on
    bnez  t0, .hang #if not 0 then hang

    # stack
    la    sp, __stack_top

    la    t0, __bss_start
    la    t1, __bss_end
.bss_loop:
    #some bs to clear the ram so zeroes are zero and not random garbage
    bgeu  t0, t1, .bss_done
    sd    zero, 0(t0)
    addi  t0, t0, 8
    j     .bss_loop
.bss_done:

    # rusting
    call  main

#infintie loop
.hang:
    wfi
    j     .hang