mod frame_alloc;
mod heap;
mod paging;

use core::{alloc::Layout, sync::atomic::AtomicU64};
use frame_alloc::FRAME_ALLOCATOR;
use limine::{LimineMemmapEntry, NonNullPtr};
use linked_list_allocator::LockedHeap;

#[global_allocator]
pub static PRESTIGE_ALLOC: LockedHeap = LockedHeap::empty();

pub static PHYSICAL_MEMORY_OFFSET: AtomicU64 = AtomicU64::new(0);

pub fn init(mem_map: &'static mut [NonNullPtr<LimineMemmapEntry>]) {
    let mut mapper = unsafe { paging::init() };
    unsafe { FRAME_ALLOCATOR.init(mem_map) };

    heap::init(&mut mapper).expect("Failed to initialize heap");
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
