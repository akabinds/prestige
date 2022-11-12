#![no_std]
#![feature(decl_macro, abi_x86_interrupt, alloc_error_handler)]
#![allow(dead_code)]
#![allow(clippy::from_over_into)]

extern crate alloc;

pub mod kernel;
mod usr;

use kernel as k;

pub fn init() {
    k::gdt::gdt_init();
    k::interrupts::int_init();
}
