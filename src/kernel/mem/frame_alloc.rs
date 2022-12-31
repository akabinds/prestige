use limine::{LimineMemmapEntry, LimineMemoryMapEntryType, NonNullPtr};
use x86_64::{structures::paging::PhysFrame, PhysAddr, VirtAddr};

pub(super) struct BootInfoFrameAllocator {
    mem_map: &'static mut [NonNullPtr<LimineMemmapEntry>],
}

impl BootInfoFrameAllocator {
    pub(super) unsafe fn init(mem_map: &'static mut [NonNullPtr<LimineMemmapEntry>]) -> Self {
        Self { mem_map }
    }

    // fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
    //     let regions = self.mem_map.iter();
    //     let usable_regions = regions.filter(|r| r.typ == LimineMemoryMapEntryType::Usable);
    //     let addr_ranges = usable_regions.map(|r| ());
    //     let frame_addrs = addr_ranges.flat_map(|r| r.step_by(4096));

    //     frame_addrs.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    // }
}
