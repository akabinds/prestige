use limine::{LimineMemmapEntry, NonNullPtr};
use linked_list_allocator::LockedHeap;
use x86_64::VirtAddr;

mod frame_alloc;
mod paging;

use frame_alloc::BootInfoFrameAllocator;

#[global_allocator]
pub static PRESTIGE_ALLOC: LockedHeap = LockedHeap::empty();

pub static mut PHYSICAL_MEMORY_OFFSET: VirtAddr = VirtAddr::zero();

pub fn init(mem_map: &'static mut [NonNullPtr<LimineMemmapEntry>]) {
    let mut mapper = unsafe { paging::init() };
    let mut frame_alloc = unsafe { BootInfoFrameAllocator::init(mem_map) };
}
