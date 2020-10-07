#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(not(test))]
use core::panic::PanicInfo;

#[macro_use]
pub mod serial;
#[macro_use]
pub mod vga_writer;


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
    println!("booting $afdone !");
    qemu_println!("QEMU connected");

    #[cfg(test)]
    test_main();

    println!("this is a $0anice string lol");
    loop {}
}
