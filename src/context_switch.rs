use core::arch::global_asm;
global_asm!(
    r#"
    .global activate
    activate:
        mrs ip, xPSR
        push {{r4, r5, r6, r7, r8, r9, r10, r11, ip, lr}}

        msr psp, r0
        mov r0, #3
        msr control, r0
        
        pop {{r4, r5, r6, r7, r8, r9, r10, r11, lr}}

        bx lr
    "#
);

