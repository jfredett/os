use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test_case]
    fn new_creates_color_correctly() {
        let c = ColorCode::new(Color::Green, Color::Black);
        assert_eq!(c, ColorCode(2));
    }
}
