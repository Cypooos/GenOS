// src/main.rs
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(genos::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use genos::{done, info};
use genos::interface::default::default_interface;
use bootloader::{entry_point, BootInfo};

extern crate alloc;


#[no_mangle]
pub fn entry_fct(boot_info: &'static BootInfo) -> ! {

    info!("Main called");

    genos::stage1();

    genos::stage2(boot_info);

    done!("OS launched sucessfully");
    #[cfg(test)]
    test_main();


    default_interface();
    //genos::hlt_loop();
}

entry_point!(entry_fct);

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    genos::error!("PANIC:\n{}", info);
    genos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    genos::testing::panic_handler(info);
    // genos::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
