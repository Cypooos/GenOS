#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::vga_writer::_print(format_args!($($arg)*));
        #[cfg(feature = "qemu-connect")]
        {$crate::serial::_print(format_args!($($arg)*));}
    });
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        match ($crate::OS_INFO.lock().boot_level) {
            0 => {
                $crate::vga_writer::_print(format_args!("[$05DBUG$!] "));
                $crate::vga_writer::_print(format_args!($($arg)*));
                $crate::vga_writer::_print(format_args!("\n"));
            },
            1 => {
                $crate::TUI::desktop::DESKTOP.lock().print(format_args!("[$05DBUG$!] "));
                $crate::TUI::desktop::DESKTOP.lock().print(format_args!($($arg)*));
                $crate::TUI::desktop::DESKTOP.lock().print((format_args!("\n")));
            },
            _ => {}
        };
        #[cfg(feature = "qemu-connect")]
        {
            $crate::serial::_print(format_args!("[\x1b[0;35mDBUG\x1b[0m] "));
            $crate::serial::_print(format_args!($($arg)*));
            $crate::serial::_print(format_args!("\n"));
        }
    });
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        match ($crate::OS_INFO.lock().boot_level) {
            0 => {
                $crate::vga_writer::_print(format_args!("[$04ERRO$!] "));
                $crate::vga_writer::_print(format_args!($($arg)*));
                $crate::vga_writer::_print(format_args!("\n"));
            },
            1 => {
                $crate::TUI::desktop::DESKTOP.lock().print((format_args!("[$04ERRO$!] ")));
                $crate::TUI::desktop::DESKTOP.lock().print(format_args!($($arg)*));
                $crate::TUI::desktop::DESKTOP.lock().print((format_args!("\n")));
            },
            _ => {}
        };
        #[cfg(feature = "qemu-connect")]
        {
        $crate::serial::_print(format_args!("[\x1b[0;31mERRO\x1b[0m] "));
        $crate::serial::_print(format_args!($($arg)*));
        $crate::serial::_print(format_args!("\n"));
        }
    });
}

#[macro_export]
macro_rules! done {
    ($($arg:tt)*) => ({
        match ($crate::OS_INFO.lock().boot_level) {
            0 => {
                $crate::vga_writer::_print(format_args!("[$0ADONE$!] "));
                $crate::vga_writer::_print(format_args!($($arg)*));
                $crate::vga_writer::_print(format_args!("\n"));
            },
            1 => {
                $crate::TUI::desktop::DESKTOP.lock().print((format_args!("[$0ADONE$!] ")));
                $crate::TUI::desktop::DESKTOP.lock().print(format_args!($($arg)*));
                $crate::TUI::desktop::DESKTOP.lock().print((format_args!("\n")));
            },
            _ => {}
        };
        #[cfg(feature = "qemu-connect")]
        {
            $crate::serial::_print(format_args!("[\x1b[0;32mDONE\x1b[0m] "));
            $crate::serial::_print(format_args!($($arg)*));
            $crate::serial::_print(format_args!("\n"));
        }
    });
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ({
        match ($crate::OS_INFO.lock().boot_level) {
            0 => {
                $crate::vga_writer::_print(format_args!("[$0EWARN$!] "));
                $crate::vga_writer::_print(format_args!($($arg)*));
                $crate::vga_writer::_print(format_args!("\n"));
            },
            1 => {
                $crate::TUI::desktop::DESKTOP.lock().print((format_args!("[$0EWARN$!] ")));
                $crate::TUI::desktop::DESKTOP.lock().print(format_args!($($arg)*));
                $crate::TUI::desktop::DESKTOP.lock().print((format_args!("\n")));
            },
            _ => {}
        };
        #[cfg(feature = "qemu-connect")]
        {
            $crate::serial::_print(format_args!("[\x1b[0;33mWARN\x1b[0m] "));
            $crate::serial::_print(format_args!($($arg)*));
            $crate::serial::_print(format_args!("\n"));
        }
    });
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ({
        match ($crate::OS_INFO.lock().boot_level) {
            0 => {
                $crate::vga_writer::_print(format_args!("[$03INFO$!] "));
                $crate::vga_writer::_print(format_args!($($arg)*));
                $crate::vga_writer::_print(format_args!("\n"));
            },
            1 => {
                $crate::TUI::desktop::DESKTOP.lock().print((format_args!("[$03INFO$!] ")));
                $crate::TUI::desktop::DESKTOP.lock().print(format_args!($($arg)*));
                $crate::TUI::desktop::DESKTOP.lock().print((format_args!("\n")));
            },
            _ => {}
        };
        #[cfg(feature = "qemu-connect")]
        {
            $crate::serial::_print(format_args!("[\x1b[0;36mINFO\x1b[0m] "));
            $crate::serial::_print(format_args!($($arg)*));
            $crate::serial::_print(format_args!("\n"));
        }
    });
}
