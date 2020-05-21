#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use os::{self, println, serial_println, serial_print};

use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    serial_println!("Running {}", file!());
    os::init();

    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    serial_println!("Running basic_boot.rs");
    serial_print!("Testing println! macro... ");
    println!("test output, please ignore.");
    serial_println!("[ok]");
}


