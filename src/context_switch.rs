use core::arch::global_asm;
global_asm!(
    r#"
    .global activate
    activate:
        // Save kernel state
        // r0-r3, r12, LR, PC, xPSR is automatically pushed after the exception 
        // Store the rest of the registers
        // ip = r12
        mrs ip, xPSR
        push {{r4, r5, r6, r7, r8, r9, r10, r11, ip, lr}}

        // Switch to PSP and unprivileged mode by setting the last 2 bits of the CONTROL register
        msr psp, r0
        mov r0, #3
        msr control, r0
        
        // Load user state
        pop {{r4, r5, r6, r7, r8, r9, r10, r11, lr}}

        bx lr
    "#
);
