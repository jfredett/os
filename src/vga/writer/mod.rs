use super::*;

mod fmt_write;

pub struct Writer {
    pub(crate) col: usize,
    pub(crate) color: ColorCode,
    pub(crate) buffer: &'static mut Buffer
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col >= BUFF_WIDTH {
                    self.new_line();
                }

                let row = BUFF_HEIGHT - 1;
                let col = self.col;
                let color = self.color;

                self.buffer.buf[row][col].write(ScreenChar::new(byte, color));
                self.col += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe)
            }
        }
    }

    fn new_line(&mut self) {
        // shift all lines up by 1
        for row in 1..BUFF_HEIGHT {
            for col in 0..BUFF_WIDTH {
                let character = self.buffer.buf[row][col].read();
                self.buffer.buf[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFF_HEIGHT - 1); // clear bottom row
        self.col = 0; // carriage return
    }

    fn clear_row(&mut self, row: usize) { 
        for col in 0..BUFF_WIDTH {
            self.buffer.buf[row][col].write(
                ScreenChar::new(b' ', self.color)
            );
        }
    }
}

