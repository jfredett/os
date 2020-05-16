#![feature(asm)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga;

use vga::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    let mut i = 0;
    loop {
        writeln!(WRITER.lock(), "i = {}; i^2 = {}", i, i*i).unwrap();
        i += 1;
    }
}
