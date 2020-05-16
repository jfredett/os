#![feature(asm, custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga;
mod qemu_exit_code;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

cfg_if::cfg_if! {
    if #[cfg(test)] {
        mod test_runner;
        use test_runner::test_runner;
        use qemu_exit_code::*;

        #[no_mangle]
        pub extern "C" fn _start() {
            test_main();
            println!("Testing complete.");
            exit_qemu(QemuExitCode::Success);
        }
    } else {
        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            main();
            loop {}
        }
    }
}

pub fn main() {
    let mut i = 0;
    loop {
        println!("i = {}; i^3 = {}", i, i*i*i);
        i += 1;
    }
}
