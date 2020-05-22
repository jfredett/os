use super::*;

pub extern "x86-interrupt" fn handler(_stack_frame: &mut InterruptStackFrame) {
    end_interrupt(InterruptIndex::Timer);
}
