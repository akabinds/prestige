use x86_64::{
    structures::paging::{mapper::MapToError, Mapper, Page, PageTableFlags, Size4KiB},
    VirtAddr,
};

pub(super) const HEAP_SIZE: usize = 128 * 1024 * 1024; // 128 GiB
pub(super) const HEAP_START: usize = 0xfffff80000000000;
pub(super) const HEAP_END: usize = HEAP_START + HEAP_SIZE - 1;

pub(super) fn init(mapper: &mut impl Mapper<Size4KiB>) -> Result<(), MapToError<Size4KiB>> {
    let heap_start = VirtAddr::new(HEAP_START as u64);
    let heap_end = VirtAddr::new(HEAP_END as u64);

    unsafe {
        super::PRESTIGE_ALLOC
            .lock()
            .init(heap_start.as_mut_ptr(), HEAP_SIZE);
    }

    Ok(())
}
