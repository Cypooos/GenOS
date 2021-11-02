#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::io::vga_writer::_print(format_args!($($arg)*));
        #[cfg(feature = "qemu-connect")]
        {$crate::io::qemu::_print(format_args!($($arg)*));}
    });
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        #[cfg(feature = "qemu-connect")]
        {
            $crate::io::qemu::_print(format_args!("[\x1b[0;35mDBUG\x1b[0m] "));
            $crate::io::qemu::_print(format_args!($($arg)*));
            $crate::io::qemu::_print(format_args!("\n"));
        }
        match ($crate::OS_INFO.lock().boot_level) {
            0 => {
                $crate::io::vga_writer::_print(format_args!("[$05DBUG$!] "));
                $crate::io::vga_writer::_print(format_args!($($arg)*));
                $crate::io::vga_writer::_print(format_args!("\n"));
            },
            1 => {
                $crate::game::desktop::_print(format_args!("[$05DBUG$!] "));
                $crate::game::desktop::_print(format_args!($($arg)*));
                $crate::game::desktop::_print((format_args!("\n")));
            },
            _ => {}
        };
    });
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        #[cfg(feature = "qemu-connect")]
        {
        $crate::io::qemu::_print(format_args!("[\x1b[0;31mERRO\x1b[0m] "));
        $crate::io::qemu::_print(format_args!($($arg)*));
        $crate::io::qemu::_print(format_args!("\n"));
        }
        match ($crate::OS_INFO.lock().boot_level) {
            0 => {
                $crate::io::vga_writer::_print(format_args!("[$04ERRO$!] "));
                $crate::io::vga_writer::_print(format_args!($($arg)*));
                $crate::io::vga_writer::_print(format_args!("\n"));
            },
            1 => {
               $crate::game::desktop::_print((format_args!("[$04ERRO$!] ")));
               $crate::game::desktop::_print(format_args!($($arg)*));
               $crate::game::desktop::_print((format_args!("\n")));
            },
            _ => {}
        };
    });
}

#[macro_export]
macro_rules! done {
    ($($arg:tt)*) => ({
        #[cfg(feature = "qemu-connect")]
        {
            $crate::io::qemu::_print(format_args!("[\x1b[0;32mDONE\x1b[0m] "));
            $crate::io::qemu::_print(format_args!($($arg)*));
            $crate::io::qemu::_print(format_args!("\n"));
        }
        match ($crate::OS_INFO.lock().boot_level) {
            0 => {
                $crate::io::vga_writer::_print(format_args!("[$0ADONE$!] "));
                $crate::io::vga_writer::_print(format_args!($($arg)*));
                $crate::io::vga_writer::_print(format_args!("\n"));
            },
            1 => {
                $crate::game::desktop::_print((format_args!("[$0ADONE$!] ")));
                $crate::game::desktop::_print(format_args!($($arg)*));
                $crate::game::desktop::_print((format_args!("\n")));
            },
            _ => {}
        };
    });
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ({
        #[cfg(feature = "qemu-connect")]
        {
            $crate::io::qemu::_print(format_args!("[\x1b[0;33mWARN\x1b[0m] "));
            $crate::io::qemu::_print(format_args!($($arg)*));
            $crate::io::qemu::_print(format_args!("\n"));
        }
        match ($crate::OS_INFO.lock().boot_level) {
            0 => {
                $crate::io::vga_writer::_print(format_args!("[$0EWARN$!] "));
                $crate::io::vga_writer::_print(format_args!($($arg)*));
                $crate::io::vga_writer::_print(format_args!("\n"));
            },
            1 => {
                $crate::game::desktop::_print((format_args!("[$0EWARN$!] ")));
                $crate::game::desktop::_print(format_args!($($arg)*));
                $crate::game::desktop::_print((format_args!("\n")));
            },
            _ => {}
        };
    });
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ({
        #[cfg(feature = "qemu-connect")]
        {
            $crate::io::qemu::_print(format_args!("[\x1b[0;36mINFO\x1b[0m] "));
            $crate::io::qemu::_print(format_args!($($arg)*));
            $crate::io::qemu::_print(format_args!("\n"));
        }
        match ($crate::OS_INFO.lock().boot_level) {
            0 => {
                $crate::io::vga_writer::_print(format_args!("[$03INFO$!] "));
                $crate::io::vga_writer::_print(format_args!($($arg)*));
                $crate::io::vga_writer::_print(format_args!("\n"));
            },
            1 => {
                $crate::game::desktop::_print((format_args!("[$03INFO$!] ")));
                $crate::game::desktop::_print(format_args!($($arg)*));
                $crate::game::desktop::_print((format_args!("\n")));
            },
            _ => {}
        };
    });
}
