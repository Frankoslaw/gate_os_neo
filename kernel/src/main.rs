#![no_std]
#![no_main]

#[macro_use]
extern crate lazy_static;

use core::panic::PanicInfo;
use limine::request::FramebufferRequest;

mod arch;
mod framebuffer;

static FRAMEBUFFER: FramebufferRequest = FramebufferRequest::new();

pub fn main() -> ! {
    if let Some(framebuffer_response) = FRAMEBUFFER.get_response() {
        let framebuffer = match framebuffer_response.framebuffers().next() {
            Some(i) => i,
            None => arch::halt_loop(),
        };

        framebuffer::init(&framebuffer);
    }

    arch::halt_loop();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
