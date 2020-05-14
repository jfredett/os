use core::fmt;
use volatile::Volatile;

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
