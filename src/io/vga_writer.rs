use alloc::vec::Vec;
use volatile::Volatile;

use core::{fmt, iter::Scan};
use lazy_static::lazy_static;
use spin::Mutex;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]

// 80 * 25
// $ back char

pub enum Color {
    Black = 0,       // 0
    Blue = 1,        // 1
    Green = 2,       // 2
    Cyan = 3,        // 3
    Red = 4,         // 4
    Magenta = 5,     // 5
    Brown = 6,       // 6
    LightGray = 7,   // 7
    DarkGray = 8,    // 8
    LightBlue = 9,   // 9
    LightGreen = 10, // A
    LightCyan = 11,  // B
    LightRed = 12,   // C
    Pink = 13,       // D
    Yellow = 14,     // E
    White = 15,      // F
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorCode {
    back: Option<Color>,
    front: Option<Color>,
}

impl ColorCode {
    #[allow(dead_code)]
    fn new(back: Option<Color>, front: Option<Color>) -> Self {
        ColorCode { back, front }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: u8,
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;
pub const DEFAULT_BACK: Color = Color::Blue;
pub const DEFAULT_FRONT: Color = Color::White;
pub const DEFAULT_COLOR_CODE: ColorCode = ColorCode {
    back: Some(DEFAULT_BACK),
    front: Some(DEFAULT_FRONT),
};
pub const DEFAULT_SCREENCHAR: ScreenChar = ScreenChar {
    ascii_character: ' ' as u8,
    color_code: (DEFAULT_BACK as u8) << 4 | (DEFAULT_FRONT as u8),
};

pub fn color_code_to_repr(a: ColorCode) -> u8 {
    (a.back.unwrap_or(DEFAULT_BACK) as u8) << 4 | (a.front.unwrap_or(DEFAULT_FRONT) as u8)
}

#[repr(transparent)]
pub struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    position: (usize, usize),
    pub new_line_padding: usize,
    pub color_code: ColorCode,
    buffer: &'static mut Buffer,
    pub is_limited: bool,
}

impl Writer {
    #[allow(dead_code)]
    pub fn set_color(&mut self, back: Option<Color>, front: Option<Color>) {
        self.color_code = ColorCode::new(back, front);
    }

    pub fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
        self.position = (0, 0);
    }

    pub fn write_byte(&mut self, byte: u8) {
        //TODO: Transparency to write_byte
        match byte {
            b'\n' => self.scroll(),
            byte => {
                if self.position.0 >= BUFFER_WIDTH {
                    self.scroll();
                }

                let row = self.position.1;
                let col = self.position.0;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: color_code_to_repr(color_code), // Just do the default if none
                });
                self.position.0 += 1;
            }
        }
    }

    fn scroll(&mut self) {
        if self.position.1 >= BUFFER_HEIGHT - 1 {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(character);
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
            self.position.0 = 0;
        } else {
            self.position.0 = 0;
            self.position.1 += 1;
        }
    }

    pub fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(DEFAULT_SCREENCHAR);
        }
    }

    // NO ALLOCATOR !!!!!!
    pub fn write_string(&mut self, s: &str) {
        /*
        act_pos, act_color
        char by char:
            if ($,$) -> draw $
            if ($,*) -> draw color
            if ($,a,b) -> try get a,b in [!,~,A-F,0-9]; change act_color; pos +3
            if \n -> act_pos.y+1, act_pos.x = self.pos
            if T ->
        */

        let mut text = s.chars();

        macro_rules! read_mem {
            () => {
                self.buffer.chars[self.position.1][self.position.0].read()
            };
        }

        macro_rules! write_mem {
            ($ascii:expr,$color:expr) => {{
                let color = match ($color.back, $color.front) {
                    (Some(a), Some(b)) => (a as u8) << 4 | (b as u8),
                    (Some(a), None) => (a as u8) << 4 | read_mem!().color_code & 0x0F,
                    (None, Some(b)) => read_mem!().color_code & 0xF0 | b as u8,
                    (None, None) => read_mem!().color_code,
                };

                self.buffer.chars[self.position.1][self.position.0].write(ScreenChar {
                    ascii_character: $ascii,
                    color_code: color,
                });
            }};
        }
        // pikeable by x iterator
        let mut act = text.next();
        while let Some(x) = act {
            // is out of bounds ?
            if self.position.0 >= 80 {
                self.position = (self.new_line_padding, self.position.1 + 1);
            }
            if self.position.1 >= 25 {
                self.position = (self.new_line_padding, 0); // loop around
            }

            match x {
                // $
                '$' => match text.next() {
                    None => {
                        // $,END
                        write_mem!(b'$', self.color_code);
                        self.position.0 += 1;
                    }
                    Some('$') => {
                        // $,$
                        write_mem!(b'$', self.color_code);
                        self.position.0 += 1;
                    }
                    Some('*') => {
                        // $,*
                        write_mem!(read_mem!().ascii_character, self.color_code);
                        self.position.0 += 1;
                    }
                    Some(a) => {
                        // $,A
                        if let Some(b) = text.next() {
                            // $,A,B

                            let trans = |a: char, is_first: bool| {
                                match a {
                                    '0' => Some(Color::Black),
                                    '1' => Some(Color::Blue),
                                    '2' => Some(Color::Green),
                                    '3' => Some(Color::Cyan),
                                    '4' => Some(Color::Red),
                                    '5' => Some(Color::Magenta),
                                    '6' => Some(Color::Brown),
                                    '7' => Some(Color::LightGray),
                                    '8' => Some(Color::DarkGray),
                                    '9' => Some(Color::LightBlue),
                                    'A' | 'a' => Some(Color::LightGreen),
                                    'B' | 'b' => Some(Color::LightCyan),
                                    'C' | 'c' => Some(Color::LightRed),
                                    'D' | 'd' => Some(Color::Pink),
                                    'E' | 'e' => Some(Color::Yellow),
                                    'F' | 'f' => Some(Color::White),
                                    '~' => Some(if is_first {
                                        DEFAULT_BACK
                                    } else {
                                        DEFAULT_FRONT
                                    }),
                                    '!' => None,
                                    _ => Some(Color::Red), // red if error
                                }
                            };
                            self.color_code = ColorCode::new(trans(a, true), trans(b, false))
                        } else {
                            // $,A,END
                        }
                    }
                },
                // \n,
                '\n' => self.position = (self.new_line_padding, self.position.1 + 1),
                // A,
                a => {
                    write_mem!(a as u8, self.color_code);
                    self.position.0 += 1;
                }
            };
            act = text.next();
        }
    }

    /*
    pub fn old_write_string(&mut self, s: &str) {
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
                match (ia, iter.next()) {
                    (Some(a), Some(b)) => {
                        let mut color = [0; 1];
                        let color_a = if a == '!' {
                            Ok(DEFAULT_BACK as u8)
                        } else {
                            u8::from_str_radix((a).encode_utf8(&mut color), 16)
                        };
                        let color_b = if b == '!' {
                            Ok(DEFAULT_FRONT as u8)
                        } else {
                            u8::from_str_radix((b).encode_utf8(&mut color), 16)
                        };
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
    } */
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
        new_line_padding: 0,
        color_code: DEFAULT_COLOR_CODE,
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        is_limited: false
    });
}

#[macro_export]
macro_rules! vga_println {
    () => ($crate::vga_print!("\n"));
    ($($arg:tt)*) => ($crate::vga_print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! vga_print {
    ($($arg:tt)*) => ($crate::io::vga_writer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! vga_write {
    ($x:expr,$y:expr, $($arg:tt)*) => ($crate::io::vga_writer::_write(($x,$y),format_args!($($arg)*)));
}

#[macro_export]
macro_rules! vga_colors {
    ($x:expr,$y:expr) => {
        $crate::io::vga_writer::WRITER.lock().set_color($x, $y)
    };
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
        let save = WRITER.lock().position;
        WRITER.lock().position = pos;
        WRITER.lock().new_line_padding = pos.0;
        WRITER.lock().write_fmt(args).unwrap();
        WRITER.lock().position = save;
        WRITER.lock().new_line_padding = 0;
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
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}
