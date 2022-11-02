#![no_std]
#![no_main]
#![feature(decl_macro)]

mod kernel;
mod vga;

use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn _start() -> ! {
    println!("Hello World!");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
