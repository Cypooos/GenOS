#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(genos::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use genos::{println,print};
use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[test_case]
fn boot_empty () {
}

#[test_case]
fn print_colored() {
    print!("<");
    print!("$0agreen $$ $ok $!reset $o");
    print!(">");
}


#[test_case]
fn print_scroll() {
    for _ in 0..50 {
        println!("scrolllllll.....");
    }
}

#[test_case]
fn assertion() {
    assert_eq!(1, 1);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    genos::testing::panic_handler(info)
}
