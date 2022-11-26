#![no_std]
#![feature(decl_macro, abi_x86_interrupt, alloc_error_handler, naked_functions)]
#![allow(dead_code, unused_imports, unused_variables, unused_mut)]
#![allow(
    clippy::from_over_into,
    clippy::missing_safety_doc,
    clippy::new_without_default
)]

extern crate alloc;

pub mod err;
pub mod kernel;
pub mod usr;

use bootloader::BootInfo;
use kernel as k;

pub fn init(boot_info: &'static BootInfo) {
    k::io::vga::init();
    k::gdt::init();
    k::interrupts::init();
    k::io::serial::init();
    k::io::keyboard::init();

    k::mem::init(boot_info);
}
