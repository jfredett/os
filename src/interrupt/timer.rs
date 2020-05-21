use super::*;

pub extern "x86-interrupt" fn handler(_stack_frame: &mut InterruptStackFrame) {
    print!(".");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(
                InterruptIndex::Timer.as_u8()
            );
    }
}
