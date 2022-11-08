#![no_std]
#![no_main]
#![feature(decl_macro, abi_x86_interrupt, alloc_error_handler, format_args_nl)]
#![allow(dead_code)]

extern crate alloc;

mod kernel;

use core::panic::PanicInfo;
use kernel::{
    hlt_loop,
    io::stdout::println,
    Initialize,
};
use x86_64::structures::{idt::InterruptDescriptorTable, gdt::GlobalDescriptorTable};

macro init_structures($($s:ty),+) {
    $(
        <$s>::init();
    )+
}

#[no_mangle]
extern "C" fn _start() -> ! {
    println!("Hello World!");

    init_structures!(InterruptDescriptorTable, GlobalDescriptorTable);

    println!("debug print reached");

    hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}
