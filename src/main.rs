#![no_std] // Don't link STD library
#![no_main] // Disable all Rust-Level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;
mod serial;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}



#[no_mangle] // Won't mangle name of the function (The name of the fn will be _start and will not be converted to anything else)
pub extern "C" fn _start() -> ! {
    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello from _start\n\n").unwrap();
    // write!(vga_buffer::WRITER.lock(), "You are the Kernel Run #{} of this year. You got rewarded with {} penies.", 42, 2).unwrap();

    // This is a reimplementation of println! (src/vga_buffer.rs)
    println!("You are the Kernel Run #{} of this year. You got rewarded with 2 penies.", 42);
    // panic!("AH! I panicked...");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_test() {
    serial_print!("First Kernel test: ");
    assert_eq!(1,1);
    serial_println!("[ok]")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4); //The 'iobase' that we specified in config.toml
        port.write(exit_code as u32)
    }
}