#![no_std]
#![no_main]
#![feature(decl_macro, abi_x86_interrupt)]
#![allow(dead_code)]

mod kernel;

use core::panic::PanicInfo;
use kernel::{stdout::println, Initialize};
use x86_64::{self, structures::idt::InterruptDescriptorTable};

fn init() {
    InterruptDescriptorTable::init();
}

#[no_mangle]
extern "C" fn _start() -> ! {
    println!("Hello World!");

    init();

    x86_64::instructions::interrupts::int3();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
