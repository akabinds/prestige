pub mod gdt;
pub mod interrupts;
pub mod stdout;

pub trait Initialize {
    fn init();
}