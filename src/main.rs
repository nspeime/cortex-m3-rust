#![allow(unused)]
#![no_main]
#![no_std]
use core::ptr;
use core::panic::PanicInfo;
mod uart;
mod context_switch;
use uart::{uart_init, uart_print};

extern "C" {
    fn activate(task_stack: *const u32);
}

fn usertask1() {
    uart_print("USER TASK 1\n");
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    uart_init();
    let mut usertask_stack: [u32; 256] = [0; 256];
    let usertask_stack_start = &mut usertask_stack[256 - 16] as *mut u32;
    unsafe {
        ptr::write_volatile(usertask_stack_start.offset(8), usertask1 as u32);
        activate(usertask_stack_start);
    }
    loop {}
}

#[link_section = ".vector_table.isr_vectors"]
#[no_mangle]
pub static ISR_VECTORS: [unsafe extern "C" fn() -> !; 1] = [Reset];

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}
