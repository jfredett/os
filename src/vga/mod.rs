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
    WRITER.lock().write_fmt(args).unwrap();
}
