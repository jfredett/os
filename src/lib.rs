#![no_std]
#![cfg_attr(test, no_main)]
#![feature(asm, custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub use core::panic::PanicInfo;

pub mod vga;
pub mod serial;
pub mod test_runner;
pub mod qemu_exit_code;

pub use vga::*;
pub use serial::*;
pub use test_runner::*;
pub use qemu_exit_code::*;


/// test entry point (xtest)
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("Running lib.rs unit tests.");
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
