pub(crate) mod gdt;
pub(crate) mod interrupts;
pub(crate) mod mem;
pub(crate) mod syscall;

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
