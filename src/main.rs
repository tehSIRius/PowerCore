#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
/// OS entry point
/// 'Extern "C"` tells the compiler to use C calling convention
/// no_mangle preserves _start name instead of compiler given one
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
// Custom panic_handler, since previous one is inside std
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}