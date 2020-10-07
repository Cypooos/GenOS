
use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

const QEMU_PORT :u16 = 0x3F8;

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
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
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
