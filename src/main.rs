// src/main.rs
#![no_std]
#![feature(asm)]
#![feature(array_map)]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(genos::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use genos::game::audio;
use genos::{allocator, game::desktop::DESKTOP};
use genos::{debug, done, error, info}; // new import

use bootloader::{entry_point, BootInfo};

extern crate alloc;

#[no_mangle]
pub fn entry_fct(boot_info: &'static BootInfo) -> ! {
    use genos::memory::BootInfoFrameAllocator;
    use x86_64::VirtAddr;

    info!("main called");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { genos::memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    genos::stage1();

    #[cfg(test)]
    test_main();

    // PUTAIN DE LIGNE QUE j'AVAIS OUBLIEEEE
    // MEMENTO MORI
    debug!("Initialisation of the allocator");

    done!("OS launch");

    audio::set_phase(44100);
    audio::set_phase_pic2(440);

    DESKTOP.lock().start();

    genos::hlt_loop();
}

entry_point!(entry_fct);

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("PANIC:\n{}", info);
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
