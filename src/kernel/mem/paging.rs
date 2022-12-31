use super::PHYSICAL_MEMORY_OFFSET;
use x86_64::{
    registers::control::{Cr3, Cr4, Cr4Flags},
    structures::paging::{OffsetPageTable, PageTable},
    VirtAddr,
};

pub(super) unsafe fn init() -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(PHYSICAL_MEMORY_OFFSET);
    OffsetPageTable::new(level_4_table, PHYSICAL_MEMORY_OFFSET)
}

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

#[cfg(target_arch = "x86_64")]
pub(super) fn level_5_paging_enabled() -> bool {
    Cr4::read().contains(Cr4Flags::L5_PAGING)
}

#[cfg(target_arch = "aarch64")]
pub(super) const fn level_5_paging_enabled() -> bool {
    false
}
