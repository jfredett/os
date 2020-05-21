use core::fmt;
use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;

mod color;
mod color_code;
mod writer;
mod buffer;
mod screen_char;

pub use color::*;
pub use color_code::*;
pub use writer::*;
pub use buffer::*;
pub use screen_char::*;

const BUFF_HEIGHT: usize = 25;
const BUFF_WIDTH: usize = 80;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        col: 0,
        color: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts::without_interrupts;

    without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[cfg(test)]
mod test {
    use crate::*;
    use super::*;

    #[test_case]
    fn println_does_not_panic() {
        serial_print!("println! doesn't panic: ");
        println!("test content, please ignore.");
        serial_println!("[ok]");
    }

    #[test_case]
    fn println_does_not_panic_with_lots_of_output() {
        serial_print!("println! doesn't panic (large output): ");
        for _ in 0..2000 {
            println!("test content, please ignore.");
        }
        serial_println!("[ok]");
    }

    #[test_case]
    fn println_actually_prints() {
        serial_print!("println! prints content to vga buffer correctly: ");
        let s = "test content, please ignore.";
        for (idx, chr) in s.chars().enumerate() {
            let screen_char = WRITER.lock().buffer.buf[BUFF_HEIGHT -2][idx].read();
            assert_eq!(char::from(screen_char.character()), chr);
        }
        serial_println!("[ok]");
    }
}
