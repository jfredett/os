use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    character: u8,
    color: ColorCode
}

impl ScreenChar {
    pub fn new(character: u8, color: ColorCode) -> ScreenChar {
        ScreenChar { character, color }
    }
}
