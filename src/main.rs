#![no_std]
#![no_main]
#![feature(decl_macro, abi_x86_interrupt, alloc_error_handler, format_args_nl)]
#![allow(dead_code)]
#![allow(clippy::from_over_into)]

extern crate alloc;

mod kernel;

use core::panic::PanicInfo;
use kernel::{gdt::gdt_init, hlt_loop, interrupts::int_init, io::stdout::println};

fn init() {
    gdt_init();
    int_init();
}

#[no_mangle]
extern "C" fn _start() -> ! {
    println!("Hello World!");

    init();

    println!("debug print reached");

    hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}
