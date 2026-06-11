#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod sbi;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    sbi::print("Hello, World!\n");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    sbi::print("KERNEL PANIC\n");
    sbi::shutdown();
}