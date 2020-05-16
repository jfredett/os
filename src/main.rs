#![feature(asm)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut i = 0;
    loop {
        println!("i = {}; i^3 = {}", i, i*i*i);
        i += 1;
    }
}
