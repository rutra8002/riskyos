#![no_std]
#![no_main]

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}