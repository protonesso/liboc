#![no_std]
#![feature(asm)]
#![feature(global_asm)]
#![feature(linkage)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
	loop {}
}
