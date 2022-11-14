#![no_std]
#![feature(decl_macro, abi_x86_interrupt, alloc_error_handler)]
#![allow(dead_code, unused_imports, unused_variables)]
#![allow(clippy::from_over_into, clippy::missing_safety_doc)]

extern crate alloc;

pub mod kernel;
mod usr;

use bootloader::BootInfo;
use kernel as k;

pub fn init(boot_info: &'static BootInfo) {
    k::gdt::gdt_init();
    k::interrupts::int_init();

    k::mem::init(boot_info);
}
