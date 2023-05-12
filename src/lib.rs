#![no_std]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![cfg_attr(test, no_main)]
#![test_runner(testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[allow(unused_imports)]
use core::panic::PanicInfo;

#[cfg(test)]
use bootloader::entry_point;

extern crate alloc;

#[macro_use]
pub mod io;
#[macro_use]
pub mod interface;
#[macro_use]
pub mod hdw;
#[macro_use]
pub mod testing;

// #[macro_use]
// pub mod tui;
use lazy_static::lazy_static;
use spin::Mutex;
use bootloader::BootInfo;

lazy_static! {
    pub static ref OS_INFO: Mutex<OsInfoStruct> = Mutex::new(OsInfoStruct { boot_level: 0u8 });
}

pub struct OsInfoStruct {
    pub boot_level: u8,
}

#[cfg(test)]
#[no_mangle]
pub fn entry_fct(t: &'static BootInfo) -> ! {
    stage1();
    stage2(t);
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

pub fn stage2(boot_info:&'static BootInfo) {
    use x86_64::VirtAddr;
    use hdw::memory::BootInfoFrameAllocator;
    use hdw::allocator;

    debug!("Stage 2...");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    debug!("Starting memory mapper");
    let mut mapper = unsafe { hdw::memory::init(phys_mem_offset) };
    debug!("Starting frame allocator");
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    debug!("Starting heap");
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");
    done!("Stage 2");
}

pub fn stage1() {
    debug!("Stage 1...");
    hdw::gdt::init();
    hdw::interrupts::init_idt();
    debug!("Enabling interrupts");
    unsafe { hdw::interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    debug!("Disabling Cursor");

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
    // hlt_loop();
}
