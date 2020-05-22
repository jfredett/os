use super::*;

use x86_64::instructions::port::Port;
use pc_keyboard::{
    layouts,
    DecodedKey,
    HandleControl,
    Keyboard,
    ScancodeSet1
};
use spin::Mutex;

const PS2_IO_PORT: u16 = 0x60;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
}

pub extern "x86-interrupt" fn handler(_stack_frame: &mut InterruptStackFrame) {

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(PS2_IO_PORT);

    let scancode: u8 = unsafe { port.read() };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(ch) => print!("{}", ch),
                DecodedKey::RawKey(k) => print!("{:?}", k)
            }
        }
    }

    end_interrupt(InterruptIndex::Keyboard);
}

