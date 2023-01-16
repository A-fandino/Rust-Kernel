#![no_std] // Don't link STD library
#![no_main] // Disable all Rust-Level entry points
use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // Won't mangle name of the function (The name of the fn will be _start and will not be converted to anything else)
pub extern "C" fn _start() -> ! {
    // ENTRY POINT
    let vga_buffer = 0xb8000 as *mut u8;
    
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            let position = i as isize * 2;
            *vga_buffer.offset(position) = byte;
            *vga_buffer.offset(position + 1) = 0xa;
        }
    }

    loop {}
}
