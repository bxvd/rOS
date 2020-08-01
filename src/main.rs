#![no_std]  // Disable standard library
#![no_main] // Disable main entry point

extern crate rlibc; // Provides mem tools

mod vga;

use core::panic::PanicInfo;

static MESSAGE: &[u8] = b"rOS 0.1.0 OK";

#[no_mangle] // Keep _start symbol intact
pub extern "C" fn _start() -> ! {
    
    let vga_buffer = 0xb8000 as *mut u8;

    // Write to buffer
    for (i, &byte) in MESSAGE.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // Cyan
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
