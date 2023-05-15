#[macro_export]
macro_rules! println {
    () => (genos::print!("\n"));
    ($($arg:tt)*) => (genos::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! printfln {
    () => (genos::print!("\n"));
    ($(($($val:tt)*));*) => (genos::printf!(($(($($val:tt)*));*); (write "\n")));
}


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        genos::io::vga_writer::_print(format_args!($($arg)*));
        #[cfg(feature = "qemu-connect")]
        {genos::io::qemu::_print(format_args!($($arg)*));}
    });
}

#[macro_export]
macro_rules! printf {
    ($(($($val:tt)*));*) => (genos::vga_print!(($(($($val:tt)*));*)));
    // TODO: QEMU PRINTF
}


#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        #[cfg(feature = "qemu-connect")]
        {
            genos::io::qemu::_print(format_args!("[\x1b[0;35mDBUG\x1b[0m] "));
            genos::io::qemu::_print(format_args!($($arg)*));
            genos::io::qemu::_print(format_args!("\n"));
        }
        match (genos::OS_INFO.lock().boot_level) {
            0 => {
                genos::io::vga_writer::vga_printf!((front White);(write "[");(front Magenta);(write "DBUG");(front White);(write "] "));
                genos::io::vga_writer::_print(format_args!($($arg)*));
                genos::io::vga_writer::_print(format_args!("\n"));
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
        genos::io::qemu::_print(format_args!("[\x1b[0;31mERRO\x1b[0m] "));
        genos::io::qemu::_print(format_args!($($arg)*));
        genos::io::qemu::_print(format_args!("\n"));
        }
        match (genos::OS_INFO.lock().boot_level) {
            0 => {
                genos::io::vga_writer::vga_printf!((front White);(write "[");(front Red);(write "ERRO");(front White);(write "] "));
                genos::io::vga_writer::_print(format_args!($($arg)*));
                genos::io::vga_writer::_print(format_args!("\n"));
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
            genos::io::qemu::_print(format_args!("[\x1b[0;32mDONE\x1b[0m] "));
            genos::io::qemu::_print(format_args!($($arg)*));
            genos::io::qemu::_print(format_args!("\n"));
        }
        match (genos::OS_INFO.lock().boot_level) {
            0 => {
                genos::io::vga_writer::vga_printf!((front White);(write "[");(front Green);(write "DONE");(front White);(write "] "));
                genos::io::vga_writer::_print(format_args!($($arg)*));
                genos::io::vga_writer::_print(format_args!("\n"));
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
            genos::io::qemu::_print(format_args!("[\x1b[0;33mWARN\x1b[0m] "));
            genos::io::qemu::_print(format_args!($($arg)*));
            genos::io::qemu::_print(format_args!("\n"));
        }
        match (genos::OS_INFO.lock().boot_level) {
            0 => {
                genos::io::vga_writer::vga_printf!((front White);(write "[");(front Yellow);(write "WARN");(front White);(write "] "));
                genos::io::vga_writer::_print(format_args!($($arg)*));
                genos::io::vga_writer::_print(format_args!("\n"));
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
            genos::io::qemu::_print(format_args!("[\x1b[0;36mINFO\x1b[0m] "));
            genos::io::qemu::_print(format_args!($($arg)*));
            genos::io::qemu::_print(format_args!("\n"));
        }
        match (genos::OS_INFO.lock().boot_level) {
            0 => {
                genos::io::vga_writer::vga_printf!((front White);(write "[");(front Cyan);(write "INFO");(front White);(write "] "));
                genos::io::vga_writer::_print(format_args!($($arg)*));
                genos::io::vga_writer::_print(format_args!("\n"));
            },
            _ => {}
        };
    });
}
