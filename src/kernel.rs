pub mod arch;
pub mod fs;
pub mod io;
pub mod mem;
pub mod net;
pub mod syscall;

trait Initialize {
    fn init();
}

pub fn kinit() {
    #[cfg(target_arch = "x86_64")]
    {
        arch::gdt::init();
    }

    io::serial::init();
}
