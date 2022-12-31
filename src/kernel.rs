pub mod arch;
pub mod fs;
pub mod io;
pub mod mem;
pub mod net;
pub mod syscall;

trait Initialize {
    fn init();
}
