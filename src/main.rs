#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use prestige::kernel::io::{fatal, kprint, print};

#[cfg(target_arch = "x86_64")]
// the `arch` module rexports everything in the specific architecture module
// if that architecture is the target architecture
use prestige::kernel::arch::hlt_loop;

entry_point!(kmain);

fn kmain(boot_info: &'static BootInfo) -> ! {
    prestige::init(boot_info);

    // print!("\x1b[?25h");
    // print!("test");

    #[cfg(target_arch = "x86_64")]
    hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprint!("{}", info);

    #[cfg(target_arch = "x86_64")]
    hlt_loop();
}
