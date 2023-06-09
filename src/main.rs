#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(teemo::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use teemo::println;
use teemo::task::executor::Executor;
use teemo::task::keyboard;

extern crate alloc;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use teemo::allocator;
    use teemo::memory;
    use teemo::memory::BootInfoFrameAllocator;
    use teemo::task::Task;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    teemo::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // // map an unused page
    // use x86_64::structures::paging::Page;
    // let page = Page::containing_address(VirtAddr::new(0));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // // write the string `New!` to the screen through the new mapping
    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

/// This function is called on panic in non-test mode.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    teemo::hlt_loop();
}

/// This function is called on panic in test mode.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    teemo::test_panic_handler(info)
}
