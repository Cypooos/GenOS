#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[allow(unused_imports)]
use core::panic::PanicInfo;

#[macro_use]
pub mod serial;
#[macro_use]
pub mod vga_writer;
#[macro_use]
pub mod gdt;
#[macro_use]
pub mod interrupts;
#[macro_use]
pub mod memory;
#[macro_use]
pub mod logger;

#[macro_use]
pub mod testing;

pub static BOOT_LEVEL: u8 = 1;

use bootloader::{entry_point, BootInfo};

#[cfg(test)]
#[no_mangle]
pub fn entry_fct(t: &'static BootInfo) -> ! {
    stage1();
    test_main();
    hlt_loop();
}

#[cfg(test)]
entry_point!(entry_fct);

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn stage1() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() }; // new
    x86_64::instructions::interrupts::enable(); // new
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    testing::panic_handler(info);
    hlt_loop();
}
