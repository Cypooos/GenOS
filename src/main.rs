// src/main.rs

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(genos::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use genos;
use genos::qemu_println;
use genos::vga_println;
use genos::{debug, done, error, info, println, warn};

use bootloader::{entry_point, BootInfo};

#[no_mangle]
pub fn entry_fct(boot_info: &'static BootInfo) -> ! {
    genos::stage1();
    //trigger a page fault
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };

    #[cfg(test)]
    test_main();

    done!("Did not crash");

    genos::hlt_loop();
}

entry_point!(entry_fct);

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    qemu_println!("[PANIC]");
    qemu_println!("{}", info);
    vga_println!("[$04PANIC$!]");
    vga_println!("{}", info);
    genos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    genos::testing::panic_handler(info);
    genos::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
