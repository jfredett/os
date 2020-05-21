use super::*;

pub extern "x86-interrupt" fn handler(
    stack_frame: &mut InterruptStackFrame,
    _error_code: u64
) -> ! {
    println!("DOUBLE FAULT:");
    panic!("{:#?}", stack_frame);
}
