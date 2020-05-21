#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

/// Ensures we do not triplefault due to stack overflow during a double fault.

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptStackFrame, InterruptDescriptorTable};

use os::{QemuExitCode, exit_qemu, serial_print, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("Running {}: ", file!());
    os::gdt::init();
    init_test_idt();
    overflow();
    serial_println!("[fail]");
    exit_qemu(QemuExitCode::Failure);
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[allow(unconditional_recursion)]
fn overflow() {
    overflow();
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(os::gdt::DOUBLE_FAULT_STACK_INDEX);
            }
        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(_: &mut InterruptStackFrame, _: u64) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
