use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

const QEMU_PORT: u16 = 0x3F8;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(QEMU_PORT) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
#[cfg(feature = "qemu-connect")]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! qemu_print {
    ($($arg:tt)*) => {
        #[cfg(feature = "qemu-connect")]
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! qemu_println {
    () => ($crate::qemu_print!("\n"));
    ($fmt:expr) => ($crate::qemu_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::qemu_print!(
        concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! qemu_debug {
    () => (
        #[cfg(feature = "qemu-debug")]
        $crate::qemu_print!("[DEBUG SET]\n")
    );
    ($fmt:expr) => (
        #[cfg(feature = "qemu-debug")]
        $crate::qemu_print!(concat!("[DEBUG] ",$fmt, "\n"))
    );
    ($fmt:expr, $($arg:tt)*) => (
        #[cfg(feature = "qemu-debug")]
        $crate::qemu_print!(concat!("[DEBUG] ",$fmt, "\n"), $($arg)*)
    );
    ($($arg:tt)*) => (
        #[cfg(feature = "qemu-debug")]
        $crate::qemu_print!("[DEBUG] {}\n", $($arg)*)
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[inline]
pub fn outb(port: u16, data: u8) {
    unsafe {
        asm!("out dx, al",in("dx") port,in("al") data);
    };
}
/*
#[inline(always)]
pub unsafe fn outb(port: u16, val: u8) {
    asm!("outb %al, %dx" ::
        "{dx}"(port), "{al}"(val) ::
        "volatile");
}
 */

#[inline]
pub fn outw(port: u16, data: u16) {
    unsafe {
        asm!("out dx, ax",in("dx") port,in("ax") data);
    };
}

#[inline]
pub fn outl(port: u16, data: u32) {
    unsafe {
        asm!("out dx, eax",in("dx") port,in("eax") data);
    };
}

#[inline]
pub fn inb(port: u16) -> u8 {
    let out: u8;
    unsafe {
        asm!("in al, dx",in("dx") port,out("al") out);
    };
    out
}

#[inline]
pub fn inw(port: u16) -> u16 {
    let out: u16;
    unsafe {
        asm!("in ax, dx",in("dx") port,out("ax") out);
    };
    out
}

#[inline]
pub fn inl(port: u16) -> u32 {
    let out: u32;
    unsafe {
        asm!("in eax, dx",in("dx") port,out("eax") out);
    };
    out
}
