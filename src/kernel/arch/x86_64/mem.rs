pub(crate) mod allocator;

use bootloader::{
    bootinfo::{MemoryMap, MemoryRegionType},
    BootInfo,
};
use core::sync::atomic::{AtomicUsize, Ordering};
use x86_64::{
    instructions::interrupts as x86_64cint, // x86_64 crate interrupts
    registers::control::Cr3,
    structures::paging::{
        FrameAllocator, FrameDeallocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB,
    },
    PhysAddr,
    VirtAddr,
};

static ALLOCATED_FRAMES: AtomicUsize = AtomicUsize::new(0);
static mut PHYS_MEM_OFFSET: u64 = 0;
static mut MEMORY_MAP: Option<&MemoryMap> = None;

pub(crate) fn init(boot_info: &'static BootInfo) {
    x86_64cint::without_interrupts(|| {
        unsafe {
            PHYS_MEM_OFFSET = boot_info.physical_memory_offset;
            MEMORY_MAP.replace(&boot_info.memory_map);
        }

        let mut mapper = unsafe { mapper(VirtAddr::new(PHYS_MEM_OFFSET)) };
        let mut frame_alloc = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

        allocator::init_heap(&mut mapper, &mut frame_alloc).expect("Failed to initialize heap");
    });
}

unsafe fn mapper(phys_mem_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(phys_mem_offset);
    OffsetPageTable::new(level_4_table, phys_mem_offset)
}

unsafe fn active_level_4_table(phys_mem_offset: VirtAddr) -> &'static mut PageTable {
    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = phys_mem_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

struct BootInfoFrameAllocator {
    mem_map: &'static MemoryMap,
}

impl BootInfoFrameAllocator {
    unsafe fn init(mem_map: &'static MemoryMap) -> Self {
        Self { mem_map }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.mem_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addrs = addr_ranges.flat_map(|r| r.step_by(4096));

        frame_addrs.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let next = ALLOCATED_FRAMES.fetch_add(1, Ordering::SeqCst);
        self.usable_frames().nth(next)
    }
}
