#![no_std]
#![no_main]

#[macro_use]
// Lazy static initialization for VGAWriter
extern crate lazy_static;

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	VGAprintln!("{}", _info);


	loop {}
}

#[no_mangle]
/// OS entry point
/// 'Extern "C"` tells the compiler to use C calling convention
/// no_mangle preserves _start name instead of compiler given one
pub extern "C" fn _start() -> ! {
	VGAprintln!("Hello There{}", "!");
	VGAprintln!("General Kenobi!");


	loop {}
}
