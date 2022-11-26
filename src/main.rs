#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use prestige::{
    init,
    kernel::{
        hlt_loop,
        io::{fatal, print},
        multitask::executor::Executor,
    },
};

entry_point!(kmain);

fn kmain(boot_info: &'static BootInfo) -> ! {
    init(boot_info);

    print!("Hello, World!");

    let mut executor = Executor::new();
    executor.run();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    fatal!("{}", info);
    hlt_loop();
}
