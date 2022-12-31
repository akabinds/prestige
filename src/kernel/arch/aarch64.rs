use limine::{LimineHhdmRequest, LimineMemmapRequest};

static MEMMAP: LimineMemmapRequest = LimineMemmapRequest::new(0);
static HHDM: LimineHhdmRequest = LimineHhdmRequest::new(0);

pub fn init() {}

pub fn hlt_loop() -> ! {
    loop {}
}
