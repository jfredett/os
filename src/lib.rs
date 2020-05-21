#![no_std]
#![cfg_attr(test, no_main)]
#![feature(asm, abi_x86_interrupt, custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub use core::panic::PanicInfo;

pub mod vga;
pub mod serial;
pub mod test_runner;
pub mod qemu_exit_code;
pub mod interrupt;
pub mod gdt;

pub use vga::*;
pub use serial::*;
pub use test_runner::*;
pub use qemu_exit_code::*;


/// test entry point (xtest)
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("Running lib.rs unit tests.");
    init();
    test_main();
    loop {}
}

pub fn init() {
    gdt::init();
    interrupt::init_idt();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
