use core::{marker::PhantomData, ops::Range};
use limine::{LimineMemmapEntry, LimineMemoryMapEntryType, NonNullPtr};
use spin::{Mutex, Once};
use x86_64::{
    structures::paging::{FrameAllocator, FrameDeallocator, PageSize, PhysFrame, Size4KiB},
    PhysAddr,
};

pub(super) static FRAME_ALLOCATOR: LockedBootInfoFrameAllocator =
    LockedBootInfoFrameAllocator::uninit();

trait LimineMemmapEntryUsable {
    fn usable(&self) -> Option<&Self>;
}

impl LimineMemmapEntryUsable for LimineMemmapEntry {
    fn usable(&self) -> Option<&Self> {
        if let LimineMemoryMapEntryType::Usable = self.typ {
            return Some(self);
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub(super) enum MemoryRegionType {
    #[default]
    Usable,
}

#[derive(Debug)]
pub(super) struct MemoryRegion<S: PageSize = Size4KiB> {
    frame_range: Range<u64>,
    region_type: MemoryRegionType,
    _phantom: PhantomData<S>,
}

// Implementing `Into` because we will never convert back to `LimineMemmapEntry`
impl<S: PageSize> Into<MemoryRegion<S>> for &LimineMemmapEntry {
    fn into(self) -> MemoryRegion<S> {
        let page_size = S::SIZE;
        let start_addr = self.base / page_size * page_size;
        let end_addr = ((self.base + self.len - 1) / page_size) + 1;

        MemoryRegion {
            frame_range: start_addr..end_addr,
            region_type: MemoryRegionType::default(),
            _phantom: PhantomData,
        }
    }
}

// #[derive(Debug)]
// pub(super) struct MemoryRegions<S: PageSize + 'static = Size4KiB>(&'static mut [MemoryRegion<S>]);

// impl<S: PageSize> FromIterator<MemoryRegion<S>> for MemoryRegions<S> {
//     fn from_iter<T: IntoIterator<Item = MemoryRegion<S>>>(iter: T) -> Self {
//         todo!();
//     }
// }

/// A frame allocator that uses the memory map provided by the bootloader
pub(super) struct BootInfoFrameAllocator {
    // TODO: concrete type instead of trait object
    mem_map: &'static dyn Iterator<Item = MemoryRegion>,
}

impl BootInfoFrameAllocator {
    /// Initialize the frame allocator with the given memory map.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the caller must guarantee that the memory map passed is valid.
    /// In reality, only the usable frames are used.
    unsafe fn init(mem_map: &'static mut [NonNullPtr<LimineMemmapEntry>]) -> Self {
        let mem_map = mem_map
            .iter()
            .filter_map(|e| e.usable())
            .map(Into::<MemoryRegion<_>>::into);

        Self { mem_map: &mem_map } // ERROR: WILL FIX
    }
}

unsafe impl Send for BootInfoFrameAllocator {}

pub(super) struct LockedBootInfoFrameAllocator(Once<Mutex<BootInfoFrameAllocator>>);

impl LockedBootInfoFrameAllocator {
    /// Create an unintialized version of the frame allocator.
    const fn uninit() -> Self {
        Self(Once::new())
    }

    /// Initialize the inner frame allocator with the given memory map.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the caller must guarantee that the memory map passed is valid.
    /// In reality, only the usable frames are used.
    pub(super) unsafe fn init(&self, mem_map: &'static mut [NonNullPtr<LimineMemmapEntry>]) {
        self.0
            .call_once(|| Mutex::new(BootInfoFrameAllocator::init(mem_map)));
    }

    /// Get a mutable reference to the inner frame allocator.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it assumes that the `init` function has already been called.
    /// If this function is called before `init`, it will cause undefined behavior.
    unsafe fn inner_alloc(&mut self) -> &mut BootInfoFrameAllocator {
        self.0.get_mut().unwrap_unchecked().get_mut()
    }
}

unsafe impl FrameAllocator<Size4KiB> for LockedBootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        todo!();
    }
}

impl FrameDeallocator<Size4KiB> for LockedBootInfoFrameAllocator {
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame<Size4KiB>) {
        todo!();
    }
}
