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
            os::hlt_loop();
        }

        #[panic_handler]
        fn panic(info: &PanicInfo) -> ! {
            test_panic_handler(info)
        }
    } else {
        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            println!("Starting OS");
            print!("Initializing... ");
            os::init();
            println!("done.");

            os::hlt_loop();
        }

        #[panic_handler]
        fn panic(info: &PanicInfo) -> ! {
            println!("{}", info);
            os::hlt_loop();
        }
    }
}
