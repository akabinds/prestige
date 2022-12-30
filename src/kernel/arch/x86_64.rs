pub(crate) mod gdt;
pub(crate) mod interrupts;

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
