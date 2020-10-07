#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

pub mod vga_writer;
pub mod serial;

#[cfg(test)]
mod testing;

// our existing panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("booting done !");
    println!("so god damn sexy");

    #[cfg(test)]
    test_main();

    println!("loop");
    loop {}
}
