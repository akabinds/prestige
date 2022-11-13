#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use prestige::{
    init,
    kernel::{hlt_loop, io::vga::println},
};

entry_point!(kmain);

fn kmain(_boot_info: &'static BootInfo) -> ! {
    println!("Hello World!");

    init();

    hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}
