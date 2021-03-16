#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(alloc_error_handler)]

#[allow(unused_imports)]
use core::panic::PanicInfo;

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

extern crate alloc;

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
pub mod allocator;
#[macro_use]
pub mod TUI;

#[macro_use]
pub mod testing;

// #[macro_use]
// pub mod TUI;

pub static BOOT_LEVEL: u8 = 1;

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
    debug!("Stage 1...");
    gdt::init();
    interrupts::init_idt();
    debug!("Enabling interrupts");
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    done!("Stage 1");
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    testing::panic_handler(info);
    hlt_loop();
}
