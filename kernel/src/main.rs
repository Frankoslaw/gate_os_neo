#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod arch;

pub fn main() -> ! {
    arch::halt_loop();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
