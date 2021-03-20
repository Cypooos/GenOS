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

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;
pub const DEFAULT_COLOR_CODE: ColorCode = ColorCode(7);
pub const DEFAULT_SCREEN_CHAR: ScreenChar = ScreenChar {
    ascii_character: ' ' as u8,
    color_code: DEFAULT_COLOR_CODE,
};

#[repr(transparent)]
pub struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    position: (usize, usize),
    pub color_code: ColorCode,
    pub draw_buffer: &'static mut Buffer,
    pub print_buffer: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
    pub is_limited: bool,
    pub auto_update: bool,
}

impl Writer {
    #[allow(dead_code)]
    pub fn set_color(&mut self, front: Color, back: Color) {
        self.color_code = ColorCode((front as u8) << 4 | (back as u8));
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.scroll(),
            byte => {
                if self.position.0 >= BUFFER_WIDTH {
                    self.scroll();
                }

                let row = self.position.1;
                let col = self.position.0;

                self.print_buffer[row][col].ascii_character = byte;
                self.print_buffer[row][col].color_code = self.color_code;
                self.position.0 += 1;
            }
        }
    }

    pub fn write_byte_draw(&mut self, byte: u8) {
        match byte {
            b'\n' => self.scroll(),
            byte => {
                if self.position.0 >= BUFFER_WIDTH {
                    self.position.0 = 0;
                }

                let row = self.position.1;
                let col = self.position.0;

                self.draw_buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                });
                self.position.0 += 1;
            }
        }
    }

    pub fn update(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.draw_buffer.chars[row][col].write(self.print_buffer[row][col]);
            }
        }
    }

    fn scroll(&mut self) {
        if self.position.1 >= BUFFER_HEIGHT - 1 {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.print_buffer[row][col];
                    self.print_buffer[row - 1][col] = character;
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
            self.position.0 = 0;
        } else {
            self.position.0 = 0;
            self.position.1 += 1;
        }
    }

    fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.print_buffer[row][col] = DEFAULT_SCREEN_CHAR;
        }
    }

    pub fn write_string(&mut self, s: &str) {
        let mut is_the_first_one: bool = true;
        if s == "" {
            return;
        }
        if self.is_limited {
            for byte in s.bytes() {
                self.write_byte(byte);
            }
        } else {
            let mut should_igniore_next: bool = true;
            for colorized in s.split("$") {
                if colorized == "" && !is_the_first_one {
                    self.write_byte(b'$');

                    should_igniore_next = true;
                    continue;
                };
                is_the_first_one = false;
                if should_igniore_next {
                    // in case of $$a0 , empty one will set should_igniore_next, and the next one will print (aa) and will not be interpreted
                    should_igniore_next = false;
                    for byte in colorized.chars() {
                        self.write_byte(byte as u8);
                    }
                    continue;
                }
                let mut iter = colorized.chars();
                let ia = iter.next();
                if ia == Some('!') {
                    self.color_code = DEFAULT_COLOR_CODE;
                    for byte in iter {
                        self.write_byte(byte as u8);
                    }
                    continue;
                }
                match (ia, iter.next()) {
                    (Some(a), Some(b)) => {
                        let mut color = [0; 1];
                        let color_a = i32::from_str_radix((a).encode_utf8(&mut color), 16);
                        let color_b = i32::from_str_radix((b).encode_utf8(&mut color), 16);
                        match (color_a, color_b) {
                            (Ok(ca), Ok(cb)) => {
                                self.color_code = ColorCode((ca as u8) << 4 | (cb as u8));
                                for byte in iter {
                                    self.write_byte(byte as u8);
                                }
                            }
                            _ => {
                                for byte in colorized.chars() {
                                    self.write_byte(byte as u8);
                                }
                            }
                        };
                    }
                    _ => {
                        for byte in colorized.chars() {
                            self.write_byte(byte as u8);
                        }
                    }
                }
            }
        }
        if self.auto_update {
            self.update()
        }
    }

    pub fn write_string_draw(&mut self, s: &str) {
        let mut is_the_first_one: bool = true;
        if s == "" {
            return;
        }
        if self.is_limited {
            for byte in s.bytes() {
                self.write_byte(byte);
            }
        } else {
            let mut should_igniore_next: bool = true;
            for colorized in s.split("$") {
                if colorized == "" && !is_the_first_one {
                    self.write_byte(b'$');

                    should_igniore_next = true;
                    continue;
                };
                is_the_first_one = false;
                if should_igniore_next {
                    // in case of $$a0 , empty one will set should_igniore_next, and the next one will print (aa) and will not be interpreted
                    should_igniore_next = false;
                    for byte in colorized.chars() {
                        self.write_byte(byte as u8);
                    }
                    continue;
                }
                let mut iter = colorized.chars();
                let ia = iter.next();
                if ia == Some('!') {
                    self.color_code = DEFAULT_COLOR_CODE;
                    for byte in iter {
                        self.write_byte(byte as u8);
                    }
                    continue;
                }
                match (ia, iter.next()) {
                    (Some(a), Some(b)) => {
                        let mut color = [0; 1];
                        let color_a = i32::from_str_radix((a).encode_utf8(&mut color), 16);
                        let color_b = i32::from_str_radix((b).encode_utf8(&mut color), 16);
                        match (color_a, color_b) {
                            (Ok(ca), Ok(cb)) => {
                                self.color_code = ColorCode((ca as u8) << 4 | (cb as u8));
                                for byte in iter {
                                    self.write_byte(byte as u8);
                                }
                            }
                            _ => {
                                for byte in colorized.chars() {
                                    self.write_byte(byte as u8);
                                }
                            }
                        };
                    }
                    _ => {
                        for byte in colorized.chars() {
                            self.write_byte(byte as u8);
                        }
                    }
                }
            }
        }
        if self.auto_update {
            self.update()
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
        position: (0, 0),
        color_code: DEFAULT_COLOR_CODE,
        draw_buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        print_buffer: [[DEFAULT_SCREEN_CHAR; BUFFER_WIDTH]; BUFFER_HEIGHT],
        is_limited: false,
        auto_update: true,
    });
}

#[macro_export]
macro_rules! vga_println {
    () => ($crate::vga_print!("\n"));
    ($($arg:tt)*) => ($crate::vga_print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! vga_print {
    ($($arg:tt)*) => ($crate::vga_writer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! vga_write {
    ($x:expr,$y:expr, $($arg:tt)*) => ($crate::vga_writer::_write(($x,$y),format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().color_code = DEFAULT_COLOR_CODE;
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[doc(hidden)]
pub fn _write(pos: (usize, usize), args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().color_code = DEFAULT_COLOR_CODE;
        let save = WRITER.lock().position;
        WRITER.lock().position = pos;
        WRITER.lock().write_fmt(args).unwrap();
        WRITER.lock().position = save;
    });
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.draw_buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}
