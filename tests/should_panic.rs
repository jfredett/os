#![no_std]
#![no_main]

use core::panic::PanicInfo;
use os::{QemuExitCode, exit_qemu, serial_print, serial_println};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    serial_print!("Running {}: ", file!());
    os::init();
    should_fail();
    serial_println!("[fail]");
    exit_qemu(QemuExitCode::Failure);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn should_fail() {
    assert!(false);
}
