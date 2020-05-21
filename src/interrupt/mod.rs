mod breakpoint;
mod double_fault;

use super::*;
pub use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint::handler);
        idt.double_fault.set_handler_fn(double_fault::handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
