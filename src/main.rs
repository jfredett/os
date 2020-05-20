#![no_std]
#![no_main]
#![feature(asm, custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use os::*;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            serial_println!("Running main.rs unit tests.");
            test_main();
            loop {}
        }

        #[panic_handler]
        fn panic(info: &PanicInfo) -> ! {
            test_panic_handler(info)
        }
    } else {
        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            let mut i = 0;
            loop {
                println!("i = {}; i^3 = {}", i, i*i*i);
                i += 1;
            }
        }

        #[panic_handler]
        fn panic(info: &PanicInfo) -> ! {
            println!("{}", info);
            loop {}
        }
    }
}
