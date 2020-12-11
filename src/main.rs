// src/main.rs

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(genos::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use core::panic::PanicInfo;
use genos;
use genos::allocator;
use genos::{debug, done, error, info, println}; // new import

use bootloader::{entry_point, BootInfo};

extern crate alloc;

#[no_mangle]
pub fn entry_fct(boot_info: &'static BootInfo) -> ! {
    use genos::memory::{active_level_4_table, translate_addr, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    info!("main called");

    genos::stage1();

    #[cfg(test)]
    test_main();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    debug!("Listing pages entries :");
    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            debug!("L4 Entry {}: {:?}", i, entry);
        }
    }

    let mut mapper = unsafe { genos::memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
    }

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // allocate a number on the heap
    let heap_value = Box::new(41);
    debug!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    debug!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    debug!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    debug!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

    done!("Did not crash :P");

    genos::hlt_loop();
}

entry_point!(entry_fct);

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("PANIC");
    error!("{}", info);
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
