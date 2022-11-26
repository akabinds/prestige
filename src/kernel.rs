mod fs;
pub(crate) mod gdt;
pub(crate) mod interrupts;
pub mod io;
pub(crate) mod mem;
pub mod multitask;
mod net;
mod process;
mod resource;
mod syscall;

trait Initialize {
    fn init();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
