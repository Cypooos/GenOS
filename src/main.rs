#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#![reexport_test_harness_main = "test_main"]

mod vga_writer;
mod serial;
use core::panic::PanicInfo;

// in src/main.rs

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

// our existing panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) {
    serial_println!("[failed]\n");
    println!("[failed]"),
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        print!("{}... ",core::any::type_name::<T>());
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
        println!("[$0aok]");
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("booting done !");

    #[cfg(test)]
    test_main();

    println!("infinite loop incomming...");
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run(); // new
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(0, 1);
}

#[test_case]
fn colored_print_test() {
    println!("$0agreen $$ $ok $o");
}
