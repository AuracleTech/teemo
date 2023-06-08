#![no_std]
#![no_main]

use core::panic::PanicInfo;
use teemo::{exit_qemu, serial_print, serial_println, QemuExitCode};

// TEMP : A significant drawback of this approach is that it only works for a single test function.
// TEMP : Only the first #[test_case] is executed. The execution stops after the panic handler has been called once.
// TEMP : I don’t know of a good way to solve this problem, so let me know if you have an idea!
// TEMP : This is why we do not use #[test_case] here.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
