#![no_std] // Don't link STD library
#![no_main] // Disable all Rust-Level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;
// use rust_os::println;

#[no_mangle] // Won't mangle name of the function (The name of the fn will be _start and will not be converted to anything else)
pub extern "C" fn _start() -> ! {
    println!("You are the Kernel Run #{} of this year. You got rewarded with 2 penies.", 42);
    
    rust_os::init();

    // #[warn(unconditional_recursion)]
    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // stack_overflow();

    // x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("DIdn't crash!!");
    rust_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}