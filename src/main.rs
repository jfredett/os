#![feature(asm, custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

mod vga;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    if cfg!(test) {
        #[cfg(test)]
        test_main();
        println!("Testing complete.");
    } else {
        main();
    }

    loop {}
}

pub fn main() {
    let mut i = 0;
    loop {
        println!("i = {}; i^3 = {}", i, i*i*i);
        i += 1;
    }
}
