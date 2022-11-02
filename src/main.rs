#![no_std]
#![no_main]
#![feature(decl_macro, custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(crate::test::test_runner)]

mod kernel;
mod vga;

#[cfg(test)]
mod test;

use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn _start() -> ! {
    println!("Hello World!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
