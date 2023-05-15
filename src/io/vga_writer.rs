use volatile::Volatile;

use core::{fmt, convert::TryInto};
use lazy_static::lazy_static;
use spin::Mutex;
use core::convert::TryFrom;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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


impl TryFrom<u8> for Color {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, ()> {
        match v {
            x if x == Color::Black as u8 => Ok(Color::Black),
            x if x == Color::Blue as u8 => Ok(Color::Blue),
            x if x == Color::Green as u8 => Ok(Color::Green),
            x if x == Color::Cyan as u8 => Ok(Color::Cyan),
            x if x == Color::Red as u8 => Ok(Color::Red),
            x if x == Color::Magenta as u8 => Ok(Color::Magenta),
            x if x == Color::Brown as u8 => Ok(Color::Brown),
            x if x == Color::LightGray as u8 => Ok(Color::LightGray),
            x if x == Color::DarkGray as u8 => Ok(Color::DarkGray),
            x if x == Color::LightBlue as u8 => Ok(Color::LightBlue),
            x if x == Color::LightGreen as u8 => Ok(Color::LightGreen),
            x if x == Color::LightCyan as u8 => Ok(Color::LightCyan),
            x if x == Color::LightRed as u8 => Ok(Color::LightRed),
            x if x == Color::Pink as u8 => Ok(Color::Pink),
            x if x == Color::Yellow as u8 => Ok(Color::Yellow),
            x if x == Color::White as u8 => Ok(Color::White),
            _ => Err(()),
        }
    }
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
pub const DEFAULT_BACK: Color = Color::Black;
pub const DEFAULT_FRONT: Color = Color::White;
pub const DEFAULT_COLOR_CODE: ColorCode = ColorCode {
    back: Some(DEFAULT_BACK),
    front: Some(DEFAULT_FRONT),
};
pub const DEFAULT_SCREENCHAR: ScreenChar = ScreenChar {
    ascii_character: ' ' as u8,
    color_code: (DEFAULT_BACK as u8) << 4 | (DEFAULT_FRONT as u8),
};


#[repr(transparent)]
pub struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    pub position: (usize, usize),
    pub new_line_padding: usize,
    pub color_code: ColorCode,
    buffer: &'static mut Buffer
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        position: (0, 0),
        new_line_padding: 0,
        color_code: DEFAULT_COLOR_CODE,
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
    });
}


impl Writer {
    #[allow(dead_code)]
    pub fn set_color(&mut self, back: Option<Color>, front: Option<Color>) {
        self.color_code = ColorCode::new(back, front);
    }

    fn get_col_current(&self, a: ColorCode) -> u8 {
        let default = self.buffer.chars[self.position.1][self.position.0].read().color_code & 0xFF;
        (a.back.unwrap_or((default>>4).try_into().unwrap() ) as u8) << 4 | (a.front.unwrap_or((default&0xF).try_into().unwrap()) as u8)
    }

    pub fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
        self.position = (0, 0);
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

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: self.get_col_current(color_code),
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
            self.position.0 = self.new_line_padding;
        } else {
            self.position.0 = self.new_line_padding;
            self.position.1 += 1;
        }
    }

    pub fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(DEFAULT_SCREENCHAR);
        }
    }

    pub fn write_str(&mut self, stri:&str) {
        for c in stri.chars() {
            self.write_byte(c as u8);
        }
    }

    pub fn change_color_front(&mut self, color:Option<Color>) {
        self.color_code.front = color;
    }
    pub fn change_color_back(&mut self, color:Option<Color>) {
        self.color_code.back = color;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}



#[macro_export]
macro_rules! vga_print {
    ($($arg:tt)*) => (genos::io::vga_writer::_print(format_args!($($arg)*)));
}

#[macro_export]
#[doc(hidden)]
macro_rules! _vga_printf {
    () => {};
    ($writer:expr,) => {};
    ($writer:expr,()) => {};
    ($writer:expr,w $($arg:tt)*) => {{
        ($writer).write_fmt(format_args!($($arg)*)).unwrap();
    }};
    ($writer:expr,write $($arg:tt)*) => {{
        ($writer).write_fmt(format_args!($($arg)*)).unwrap();
    }};
    ($writer:expr,f trans) => {{
        ($writer).change_color_front(None);
    }};
    ($writer:expr,f $col:tt) => {{
        ($writer).change_color_front(Some(genos::io::vga_writer::Color::$col));
    }};
    ($writer:expr,front trans) => {{
        ($writer).change_color_front(None);
    }};
    ($writer:expr,front $col:tt) => {{
        ($writer).change_color_front(Some(genos::io::vga_writer::Color::$col));
    }};
    ($writer:expr,b trans) => {{
        ($writer).change_color_back(None);
    }};
    ($writer:expr,back trans) => {{
        ($writer).change_color_back(None);
    }};
    ($writer:expr,b $col:tt) => {{
        ($writer).change_color_back(Some(genos::io::vga_writer::Color::$col));
    }};
    ($writer:expr,back $col:tt) => {{
        ($writer).change_color_back(Some(genos::io::vga_writer::Color::$col));
    }}
}

pub use _vga_printf;

#[macro_export]
macro_rules! vga_printf {
    ($(($($val:tt)*));*) => {{
        use core::fmt::Write;
        use x86_64::instructions::interrupts;
        interrupts::without_interrupts(|| {
            let mut writer = genos::io::vga_writer::WRITER.lock();
            $(genos::io::vga_writer::_vga_printf!(writer,$($val)*));*
        });

    }}
}
pub use vga_printf;


#[macro_export]
macro_rules! vga_writef {
    ($x:expr;$y:expr;$(($($val:tt)*));*) => {{
        use core::fmt::Write;
        use x86_64::instructions::interrupts;
    
        interrupts::without_interrupts(|| {
            let mut writer = genos::io::vga_writer::WRITER.lock();
            let save_pos = writer.position;
            let save_pad = writer.new_line_padding;
            writer.position.0 = $x;
            writer.position.1 = $y;
            writer.new_line_padding = $x;
            $(genos::io::vga_writer::_vga_printf!(writer,$($val)*););*
            writer.position = save_pos;
            writer.new_line_padding = save_pad;
        });
    }}
}
pub use vga_writef;


#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[doc(hidden)]
pub fn _write(pos: (usize, usize), args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        let save_pos = WRITER.lock().position;
        let save_pad = WRITER.lock().new_line_padding;
        WRITER.lock().position = pos;
        WRITER.lock().new_line_padding = pos.0;
        WRITER.lock().write_fmt(args).unwrap();
        WRITER.lock().position = save_pos;
        WRITER.lock().new_line_padding = save_pad;
    });
}