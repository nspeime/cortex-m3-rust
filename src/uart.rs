use core::ptr;

const UARTBASE: usize = 0x4000_C000;
const UARTDR: usize = UARTBASE + 0x000;
const UARTFR: usize = UARTBASE + 0x018;
const UARTCTL: usize = UARTBASE + 0x030;

const UARTEN: u32 = 1 << 0; 
const TXE: u32 = 1 << 8; 
const TXFF: u32 = 1 << 5; 

pub fn uart_init() {
    unsafe {
        ptr::write_volatile(UARTCTL as *mut u32, UARTEN | TXE);
    }
}

pub fn uart_print(s: &str) {
    for byte in s.bytes() {
        unsafe {
            while ptr::read_volatile(UARTFR as *const u32) & TXFF != 0 {}
            ptr::write_volatile(UARTDR as *mut u32, byte as u32);
        }
    }
}