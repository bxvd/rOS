#![no_std]  // Disable standard library
#![no_main] // Disable main entry point

extern crate rlibc; // Provides mem tools

mod vga;

use core::panic::PanicInfo;

#[no_mangle] // Keep _start symbol intact
pub extern "C" fn _start() -> ! {
    
    use core::fmt::Write;

    vga::WRITER.lock().write_str("OK").unwrap();
    write!(vga::WRITER.lock(), " {} {}", 42, 1.0/3.0).unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
