#![allow(unused)]
#![no_main]
#![no_std]
use core::panic::PanicInfo;

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    loop {}
}

#[link_section = ".vector_table.isr_vectors"]
#[no_mangle]
pub static ISR_VECTORS: [unsafe extern "C" fn() -> !; 1] = [Reset];

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}
