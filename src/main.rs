#![no_std] // Don't link STD library
#![no_main] // Disable all Rust-Level entry points
use core::panic::PanicInfo;
mod vga_buffer;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[no_mangle] // Won't mangle name of the function (The name of the fn will be _start and will not be converted to anything else)
pub extern "C" fn _start() -> ! {
    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello from _start\n\n").unwrap();
    // write!(vga_buffer::WRITER.lock(), "You are the Kernel Run #{} of this year. You got rewarded with {} penies.", 42, 2).unwrap();

    // This is a reimplementation of println! (src/vga_buffer.rs)
    println!("You are the Kernel Run #{} of this year. You got rewarded with 2 penies.", 42);
    panic!("AH! I panicked...");
    loop {}
}

// let vga_buffer = 0xb8000 as *mut u8;

// for (i, &byte) in HELLO.iter().enumerate() {
//     unsafe {
//         let position = i as isize * 2;
//         *vga_buffer.offset(position) = byte;
//         *vga_buffer.offset(position + 1) = 0xa;
//     }
// }