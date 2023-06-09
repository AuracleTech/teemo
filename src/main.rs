#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(teemo::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use teemo::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    teemo::init(); // new

    // invoke a double fault exception
    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }
    // stack_overflow();

    // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3(); // new

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    loop {}
}

/// This function is called on panic in non-test mode.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// This function is called on panic in test mode.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    teemo::test_panic_handler(info)
}
