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
    use genos::memory::{active_level_4_table, translate_addr};
    use x86_64::structures::paging::PageTable;
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

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
    }

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
