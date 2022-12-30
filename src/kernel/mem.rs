use limine::{LimineMemmapEntry, LimineMemmapRequest, LimineMemoryMapEntryType, NonNullPtr};
use x86_64::{structures::paging::PhysFrame, VirtAddr, PhysAddr};

static mut PHYSICAL_MEMORY_OFFSET: VirtAddr = VirtAddr::zero();
static MEMMAP: LimineMemmapRequest = LimineMemmapRequest::new(0);

struct BootInfoFrameAllocator {
    mem_map: &'static [NonNullPtr<LimineMemmapEntry>],
}

impl BootInfoFrameAllocator {
    unsafe fn init(mem_map: &'static [NonNullPtr<LimineMemmapEntry>]) -> Self {
        Self { mem_map }
    }

    // fn usable_frames() -> impl Iterator<Item = PhysFrame> {
    //     let regions = MEMMAP
    //         .get_response()
    //         .get()
    //         .expect("Unable to get response")
    //         .memmap()
    //         .iter();
    //     let usable_regions = regions.filter(|r| r.typ == LimineMemoryMapEntryType::Usable);
    //     let addr_ranges = usable_regions.map(|r| ());
    //     let frame_addrs = addr_ranges.flat_map(|r| r.step_by(4096));

    //     frame_addrs.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    // }
}
