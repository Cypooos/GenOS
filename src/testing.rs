#![no_std]
#![no_main]


use core::panic::PanicInfo;

use crate::vga_writer;
use crate::serial;



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



// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    qemu_println!("[failed]\n");
    println!("[$04failed$01]");
    qemu_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {} // not executed but whatever
}

pub trait Testable {
    fn test(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn test(&self) {
        print!("{}...    ",core::any::type_name::<T>());
        qemu_print!("{}...\t", core::any::type_name::<T>());
        self();
        qemu_println!("[ok]");
        println!("[$0aok$!]");
    }
}



#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    qemu_println!("Running {} tests", tests.len());
    println!("Running {} tests", tests.len());
    for test in tests {
        test.test(); // new
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
