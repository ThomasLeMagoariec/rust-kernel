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
    use x708a::memory::translate_addr;

    println!("Hello World{}", "!");
    x708a::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
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
