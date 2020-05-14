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

    let mut writer = Writer {
        col: 0,
        color: ColorCode::new(Color::Red, Color::White),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    let mut i = 0;
    loop {
        writeln!(writer, "i = {}; i^2 = {}", i, i*i).unwrap();
        i += 1;
    }
}
