#![no_std]
#![feature(asm)]
#![feature(array_map)]
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
pub mod game;

#[macro_use]
pub mod testing;

// #[macro_use]
// pub mod tui;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref OS_INFO: Mutex<OsInfoStruct> = Mutex::new(OsInfoStruct { boot_level: 0u8 });
}

pub struct OsInfoStruct {
    pub boot_level: u8,
}

#[cfg(test)]
#[no_mangle]
pub fn entry_fct(_t: &'static BootInfo) -> ! {
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

    debug!("Disableling Cursor");

    // Exit cursor
    // use x86_64::instructions::port::Port;
    // unsafe {
    //     let mut port = Port::new(0x3D4);
    //     port.write(0x0A as u32);
    //     let mut port = Port::new(0x3D5);
    //     port.write(0x20 as u32);
    // }
    // TODO:
    //OS_INFO.lock().boot_level = 1;
    qemu_debug!("Test");
    done!("Stage 1");
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    qemu_debug!(
        "Guess I've died\
        \n    - Big Chungus"
    );
    testing::panic_handler(info);
    hlt_loop();
}
