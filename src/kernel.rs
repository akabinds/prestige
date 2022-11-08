pub mod gdt;
pub mod interrupts;
pub mod io;
pub mod mem;

pub trait Initialize {
    fn init();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
