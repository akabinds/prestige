#![no_std]
#![no_main]
#![feature(decl_macro, abi_x86_interrupt, concat_idents)]
#![allow(dead_code)]

mod kernel;

use core::panic::PanicInfo;
use kernel::{stdout::println, Initialize};
use x86_64::structures::{gdt::GlobalDescriptorTable, idt::InterruptDescriptorTable};

macro init($($t:ty),*) {
    $(
        <$t>::init();
    )*
}

#[no_mangle]
extern "C" fn _start() -> ! {
    println!("Hello World!");

    init!(InterruptDescriptorTable, GlobalDescriptorTable);

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
