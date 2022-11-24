#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use prestige::{
    init,
    kernel::{
        hlt_loop,
        io::vga::{print, println},
        multitask::executor::Executor,
    },
};

entry_point!(kmain);

fn kmain(boot_info: &'static BootInfo) -> ! {
    print!("\x1b[?25h");

    init(boot_info);

    let mut executor = Executor::new();
    executor.run();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}
