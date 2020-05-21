use super::*;

pub extern "x86-interrupt" fn handler(stack_frame: &mut InterruptStackFrame) {
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
