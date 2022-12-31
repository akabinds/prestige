#![no_std]
#![no_main]
#![feature(decl_macro, alloc_error_handler, custom_test_frameworks)]
#![test_runner(tests::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(
    dead_code,
    non_upper_case_globals,
    unused_imports,
    unused_variables,
    unused_mut
)]
#![cfg_attr(target_arch = "x86_64", feature(abi_x86_interrupt))]

extern crate alloc;

#[macro_use]
extern crate prestige_macros;

mod kernel;
#[cfg(test)]
mod tests;

// the `arch` module rexports everything in the specific architecture module
// if that architecture is the target architecture
use kernel::arch;

use core::panic::PanicInfo;
use kernel::io::println;

#[no_mangle]
fn prestige_main() -> ! {
    arch::init();

    println!("Hello, World!");

    #[cfg(test)]
    test_main();

    arch::interrupts::hlt_loop();
}

fn kernel_main_thread() {
    todo!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    arch::interrupts::hlt_loop();
}
