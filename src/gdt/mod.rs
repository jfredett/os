mod selectors;

use x86_64::{
    VirtAddr,
    instructions::{
        segmentation::set_cs,
        tables::load_tss
    },
    structures::{
        tss::TaskStateSegment,
        gdt::{
            SegmentSelector,
            GlobalDescriptorTable,
            Descriptor
        }
    }
};
use lazy_static::lazy_static;

use selectors::Selectors;

pub const DOUBLE_FAULT_STACK_INDEX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_STACK_INDEX as usize] = {
            // TODO: Will replace with proper stack assignment when the stack manager stuff is in
            // place.
            // NOTE: Because this is not a real stack, there is no guard page, and overflows will
            // corrupt memory.
            const STACK_SIZE: usize = 4096;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_s = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_s = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors::new(code_s, tss_s))
    };
}

pub fn init() {
    GDT.0.load();
    unsafe {
        set_cs(GDT.1.code());
        load_tss(GDT.1.tss());
    }
}
