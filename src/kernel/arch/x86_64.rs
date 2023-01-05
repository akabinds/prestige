mod gdt;
pub mod interrupts;

use core::sync::atomic::Ordering;

use crate::kernel::{io, mem};
use limine::{LimineHhdmRequest, LimineMemmapRequest};
use x86_64::instructions::interrupts as x86_64cint;

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

        mem::PHYSICAL_MEMORY_OFFSET
            .store(HHDM.get_response().get().unwrap().offset, Ordering::SeqCst);

        mem::init(mem_map);
        log::info!("initialized paging and heap");

        interrupts::init();
        log::info!("initialized IDT");

        // gdt::init();
        // log::info!("initialized GDT");

        io::init();
        log::info!("initialized I/O devices");
    });
}
