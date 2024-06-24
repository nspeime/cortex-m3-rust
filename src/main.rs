#![allow(unused)]
#![no_main]
#![no_std]
use core::panic::PanicInfo;
mod uart;
use uart::{uart_init, uart_print};

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    uart_init();
    uart_print("Hello World\n");
    loop {}
}

#[link_section = ".vector_table.isr_vectors"]
#[no_mangle]
pub static ISR_VECTORS: [unsafe extern "C" fn() -> !; 1] = [Reset];

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}
