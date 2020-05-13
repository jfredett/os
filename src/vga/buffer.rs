use super::*;

#[repr(transparent)]
pub struct Buffer {
    pub(crate) buf: [[Volatile<ScreenChar>; BUFF_WIDTH]; BUFF_HEIGHT]
}
