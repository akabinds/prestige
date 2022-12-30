#![no_std]
#![feature(decl_macro, abi_x86_interrupt, alloc_error_handler)]
#![allow(dead_code, unused_imports, unused_variables, unused_mut)]
#![allow(
    clippy::new_without_default,
    clippy::collapsible_else_if,
    clippy::fn_to_numeric_cast
)]

extern crate alloc;

pub mod kernel;

use bootloader::BootInfo;
use kernel as k;

pub fn init(boot_info: &'static BootInfo) {
    k::io::vga::init();

    #[cfg(target_arch = "x86_64")]
    {
        // the `arch` module rexports everything in the specific architecture module
        // if that architecture is the target architecture
        k::arch::gdt::init();
        k::arch::interrupts::init();
        k::arch::syscall::init();
    }

    k::io::serial::init();
    k::io::keyboard::init();

    #[cfg(target_arch = "x86_64")]
    {
        // the `arch` module rexports everything in the specific architecture module
        // if that architecture is the target architecture
        k::arch::mem::init(boot_info);
    }
}
