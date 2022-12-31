mod gdt;
pub mod interrupts;

use crate::kernel::mem;
use limine::{LimineHhdmRequest, LimineMemmapRequest};
use x86_64::{
    instructions::interrupts as x86_64cint, // x86_64 crate interrupts
    VirtAddr,
};

static MEMMAP: LimineMemmapRequest = LimineMemmapRequest::new(0);
static HHDM: LimineHhdmRequest = LimineHhdmRequest::new(0);

pub fn init() {
    x86_64cint::without_interrupts(|| {
        log::info!("beginning architecture specific initialization for x86_64");

        let mem_map = MEMMAP
            .get_response()
            .get_mut()
            .expect("limine: invalid memmap response")
            .memmap_mut();

        unsafe {
            mem::PHYSICAL_MEMORY_OFFSET = VirtAddr::new(HHDM.get_response().get().unwrap().offset);
        }

        mem::init(mem_map);
    });
}
