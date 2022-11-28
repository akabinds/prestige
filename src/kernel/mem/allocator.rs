use crate::k::{io::recoverable, process};
use linked_list_allocator::LockedHeap;
use x86_64::{
    structures::paging::{
        mapper::MapToError, page::PageRangeInclusive, FrameAllocator, Mapper, Page, PageTableFlags,
        Size4KiB,
    },
    VirtAddr,
};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub(crate) const HEAP_START: usize = 0x_4444_4444_0000;
pub(crate) const HEAP_SIZE: usize = 100 * 1024;

pub(super) fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_alloc: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let heap_start = VirtAddr::new(HEAP_START as u64);
    process::init((HEAP_START + HEAP_SIZE) as u64);

    let page_range = {
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);

        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_alloc
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        unsafe { mapper.map_to(page, frame, flags, frame_alloc)?.flush() };
    }

    unsafe {
        ALLOCATOR.lock().init(heap_start.as_mut_ptr(), HEAP_SIZE);
    }

    Ok(())
}

pub(crate) fn alloc(addr: u64, size: usize) -> Result<(), ()> {
    let mut mapper = unsafe { super::mapper(VirtAddr::new(super::PHYS_MEM_OFFSET)) };
    let mut frame_alloc =
        unsafe { super::BootInfoFrameAllocator::init(super::MEMORY_MAP.unwrap()) };
    let flags =
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;

    let pages = {
        let start_page = Page::containing_address(VirtAddr::new(addr));
        let end_page = Page::containing_address(VirtAddr::new(addr + (size as u64) - 1));
        Page::range_inclusive(start_page, end_page)
    };

    for page in pages {
        let Some(frame) = frame_alloc.allocate_frame() else {
            recoverable!("Unable to allocate frame for {:?}", page);
            return Err(());
        };

        unsafe {
            let Ok(mapping) = mapper.map_to(page, frame, flags, &mut frame_alloc) else {
                recoverable!("Unable to map {:?}", page);
                return Err(());
            };

            mapping.flush();
        }
    }

    Ok(())
}

pub(crate) fn free(addr: u64, size: usize) {
    let mut mapper = unsafe { super::mapper(VirtAddr::new(super::PHYS_MEM_OFFSET)) };

    let pages: PageRangeInclusive<Size4KiB> = {
        let start_page = Page::containing_address(VirtAddr::new(addr));
        let end_page = Page::containing_address(VirtAddr::new(addr + (size as u64) - 1));
        Page::range_inclusive(start_page, end_page)
    };

    for page in pages {
        if let Ok((_, mapping)) = mapper.unmap(page) {
            mapping.flush();
        } else {
            recoverable!("Unable to unmap {:?}", page);
        }
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
