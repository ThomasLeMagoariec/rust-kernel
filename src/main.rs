#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(x708a::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;
mod testing_stuff;

extern crate alloc;

use core::panic::PanicInfo;
use alloc::{boxed::Box, rc::Rc, vec::Vec};
use bootloader::{BootInfo, entry_point, bootinfo};
use x708a::memory::BootInfoFrameAllocator;
use x86_64::{VirtAddr, structures::paging::{PageTable, Page}};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x708a::memory;
    use x708a::allocator;
    use alloc::vec;
    
    println!("Hello World{}", "!");
    x708a::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");


    let heap_values = Box::new(41);
    println!("heap_value at {:p}", heap_values);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));

    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    #[cfg(test)]
    test_main();

    println!("it didn't crash!");

    x708a::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    x708a::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    x708a::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1)
}
