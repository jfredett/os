mod breakpoint;
mod double_fault;
mod index;

use super::*;

pub use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use pic8259_simple::ChainedPics;
use spin;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint
           .set_handler_fn(breakpoint::handler);

        unsafe {
            idt.double_fault
               .set_handler_fn(double_fault::handler)
               .set_stack_index(gdt::DOUBLE_FAULT_STACK_INDEX);
        }

        idt
    };
}

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(
    unsafe {
        ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) 
    }
);

pub fn init_idt() {
    IDT.load();
}
