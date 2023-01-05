use super::PHYSICAL_MEMORY_OFFSET;
use core::sync::atomic::Ordering;
use x86_64::{
    registers::control::Cr3,
    structures::paging::{OffsetPageTable, PageTable},
    VirtAddr,
};

/// Initialize the offset page table.
///
/// # Safety
///
/// This function is unsafe because the caller must guarantee that the complete physical memory is mapped
/// to the virtual memory as `physical_memory_offset`. In addition, this function must only be called once to avoid
/// undefined behavior due to aliasing mutable references.
pub(super) unsafe fn init() -> OffsetPageTable<'static> {
    let physical_memory_offset = VirtAddr::new(PHYSICAL_MEMORY_OFFSET.load(Ordering::SeqCst));

    let active_level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(active_level_4_table, physical_memory_offset)
}

/// Return a mutable reference to the active level 4 table.
///
/// # Safety
///
/// This function is unsafe because the caller must guarantee that the complete physical memory is mapped
/// to the virtual memory as `physical_memory_offset`. In addition, this function must only be called once to avoid
/// undefined behavior due to aliasing mutable references.
#[cfg(target_arch = "x86_64")]
pub(super) unsafe fn active_level_4_table(
    physical_memory_offset: VirtAddr,
) -> &'static mut PageTable {
    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr = virt.as_mut_ptr();

    &mut *page_table_ptr
}

#[cfg(target_arch = "aarch64")]
pub(super) unsafe fn active_level_4_table() -> &'static mut PageTable {
    unimplemented!()
}
