#![no_std]  // Disable standard library
#![no_main] // Disable main entry point

extern crate rlibc; // Provides mem tools

mod vga;

use core::panic::PanicInfo;

#[no_mangle] // Keep _start symbol intact
pub extern "C" fn _start() -> ! {
    
    vga::print_something();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
