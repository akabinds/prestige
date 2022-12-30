pub mod arch;
mod fs;
pub mod io;
mod net;
mod process;
mod resource;
pub mod scheduler;
pub(crate) mod syscall;

trait Initialize {
    fn init();
}
