use super::*;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Index {
    Timer = PIC_1_OFFSET
}

impl Index {
    fn as_u8(self) -> u8 { self as u8 }
    fn as_usize(self) -> usize { usize::from(self.as_u8()) }
}
