#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(genos::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use genos::allocator;
    use genos::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    genos::stage1();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    test_main();
    genos::hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    genos::testing::panic_handler(info)
}

use alloc::boxed::Box;

#[test_case]
fn simple_allocation() {
    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(13);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13);
}

use alloc::string::String;
use genos::debug;

#[test_case]
fn string_alloc() {
    let mut heap_value_1 = String::new();
    heap_value_1 += "World";
    let heap_value_2 = String::from("Hello ");
    let out = heap_value_2 + &heap_value_1 + " !";
    debug!("{}", out);
    assert_eq!(&out, "Hello World !");
}

use alloc::vec::Vec;

#[test_case]
fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

use genos::allocator::HEAP_SIZE;
// test freed memory
#[test_case]
fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}
