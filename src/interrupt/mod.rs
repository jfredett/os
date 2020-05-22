mod breakpoint;
mod double_fault;
mod interrupt_index;
mod timer;
mod keyboard;

use super::*;
use interrupt_index::InterruptIndex;

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

        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer::handler);
        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard::handler);

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

pub fn end_interrupt(idx: InterruptIndex) {
    unsafe { PICS.lock().notify_end_of_interrupt(idx.as_u8()); }
}
