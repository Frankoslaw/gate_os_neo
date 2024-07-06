use core::arch::asm;

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    start();
}

fn start() -> ! {
    crate::main();
}

#[inline(always)]
pub fn halt_loop() -> ! {
    loop {
        unsafe { asm!("wfi") }
    }
}
