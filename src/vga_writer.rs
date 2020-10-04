use volatile::Volatile;

use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;


#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    #[allow(dead_code)]
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
pub const DEFAULT_COLOR_CODE: ColorCode = ColorCode(14);

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}



pub struct Writer {
    column_position: usize,
    pub color_code: ColorCode,
    buffer: &'static mut Buffer,
    pub is_limited: bool
}



impl Writer {

    #[allow(dead_code)]
    pub fn set_color(&mut self,front:Color,back:Color) {
        self.color_code = ColorCode((front as u8) << 4 | (back as u8));
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.scroll(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.scroll();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn scroll(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(ScreenChar {
                ascii_character: b' ',
                color_code: DEFAULT_COLOR_CODE,
            });
        }
    }

    pub fn write_string(&mut self, s: &str) {
        if self.is_limited {
            for byte in s.bytes() {
                match byte {
                    // printable ASCII byte or newline
                    0x20..=0x7e | b'\n' => self.write_byte(byte),
                    // not part of printable ASCII range
                    _ => self.write_byte(b'?'),
                }
            }
        } else {
            let mut should_igniore_next:bool = false;
            for colorized in s.split("$") {
                if colorized == "" {
                    self.write_byte(b'$');
                    should_igniore_next = true;
                    continue
                }
                if should_igniore_next { // in case of $$a0 , empty one will set should_igniore_next, and the next one will print (aa) and will not be interpreted
                    should_igniore_next = false;
                    for byte in colorized.chars() {
                        self.write_byte(byte as u8);
                    }
                    continue
                }
                let mut iter = colorized.chars();

                match (iter.next(),iter.next()) {
                    (Some(a),Some(b)) => {
                        let mut color = [0 ;1];
                        let color_a = i32::from_str_radix((a).encode_utf8(&mut color), 16);
                        let color_b = i32::from_str_radix((b).encode_utf8(&mut color), 16);
                        match (color_a,color_b) {
                            (Ok(ca),Ok(cb)) => {
                                self.color_code = ColorCode ((ca as u8) <<4 | (cb as u8));
                                for byte in iter {
                                    self.write_byte(byte as u8);
                                }
                            },
                            _ => {
                                for byte in colorized.chars(){
                                    self.write_byte(byte as u8);
                                }
                            }
                        };
                    },
                    _ => {
                        for byte in colorized.chars() {
                            self.write_byte(byte as u8);
                        }
                    }
                }
            };
        }
    }
}


#[allow(dead_code)]
pub fn test_print() {
    for a in 0x0..0xFF {
        WRITER.lock().write_byte(a as u8);
    }
}


impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}


lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: DEFAULT_COLOR_CODE,
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        is_limited:false
    });
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_writer::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().color_code = DEFAULT_COLOR_CODE;
    WRITER.lock().write_fmt(args).unwrap();
    WRITER.lock().color_code = DEFAULT_COLOR_CODE;
}
