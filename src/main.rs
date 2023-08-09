#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(x708a::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;
mod testing_stuff;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point, bootinfo};
use x708a::memory::active_level_4_table;
use x86_64::{VirtAddr, structures::paging::PageTable};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::registers::control::Cr3;

    println!("Hello World{}", "!");
    x708a::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 entry {}: {:?}", i, entry);

            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + boot_info.physical_memory_offset;
            let ptr = VirtAddr::new(virt).as_mut_ptr();
            let l3_table: &PageTable = unsafe { &*ptr };

            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    println!("  L3 entry {}: {:?}", i, entry);
                }
            }
        }
    }

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
