use super::*;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("BREAKPOINT:");
    println!("{:#?}", stack_frame);
}

#[cfg(test)]
mod tests {
    use crate::{serial_print, serial_println};

    mod exceptions {
        use super::*;

        #[test_case]
        fn breakpoint_exception_works() {
            serial_print!("testing breakpoint exception: ");
            x86_64::instructions::interrupts::int3();
            serial_println!("[ok]");
        }
    }
}
