#![no_std]
#![no_main]
#![feature(decl_macro, alloc_error_handler, custom_test_frameworks)]
#![test_runner(tests::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(dead_code, non_upper_case_globals, unused_imports)]
#![cfg_attr(target_arch = "x86_64", feature(abi_x86_interrupt))]

// extern crate alloc;

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
use limine::{LimineEntryPointRequest, LiminePtr};

static _ENTRY_POINT: LimineEntryPointRequest =
    LimineEntryPointRequest::new(0).entry(LiminePtr::new(prestige_main));

#[no_mangle]
fn prestige_main() -> ! {
    println!("Hello, World!");

    #[cfg(test)]
    test_main();

    arch::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    arch::hlt_loop();
}
