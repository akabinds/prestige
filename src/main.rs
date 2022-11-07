#![no_std]
#![no_main]
#![feature(decl_macro, abi_x86_interrupt, alloc_error_handler, format_args_nl)]
#![allow(dead_code)]

extern crate alloc;

mod kernel;

use core::panic::PanicInfo;
use kernel::io::{serial::serial_println, stdout::println};

#[no_mangle]
extern "C" fn _start() -> ! {
    serial_println!("Hello World!");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
