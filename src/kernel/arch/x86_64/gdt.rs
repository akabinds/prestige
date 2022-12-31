use crate::kernel::Initialize;
use lazy_static::lazy_static;
use x86_64::{
    instructions::tables::load_tss,
    registers::segmentation::{Segment, CS, DS, ES, FS, GS, SS},
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
    PrivilegeLevel, VirtAddr,
};

pub(super) fn init() {
    GlobalDescriptorTable::init();
}

const STACK_SIZE: usize = 4096 * 5;
static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
pub(super) const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub(super) const PAGE_FAULT_IST_INDEX: u16 = 1;
pub(super) const GENERAL_PROTECTION_FAULT_IST_INDEX: u16 = 2;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        tss.privilege_stack_table[0] = VirtAddr::from_ptr(unsafe { &STACK }) + STACK_SIZE;
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] =
            VirtAddr::from_ptr(unsafe { &STACK }) + STACK_SIZE;

        tss
    };
    pub static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();

        let kernel_code = gdt.add_entry(Descriptor::kernel_code_segment());
        let kernel_data = gdt.add_entry(Descriptor::kernel_data_segment());
        let tss = gdt.add_entry(Descriptor::tss_segment(&TSS));
        let user_data = gdt.add_entry(Descriptor::user_data_segment());
        let user_code = gdt.add_entry(Descriptor::user_code_segment());

        (
            gdt,
            Selectors {
                kernel_code,
                kernel_data,
                tss,
                user_data,
                user_code,
            },
        )
    };
}

pub struct Selectors {
    pub(crate) kernel_code: SegmentSelector,
    pub(crate) kernel_data: SegmentSelector,
    tss: SegmentSelector,
    pub(crate) user_data: SegmentSelector,
    pub(crate) user_code: SegmentSelector,
}

impl Initialize for GlobalDescriptorTable {
    fn init() {
        GDT.0.load();

        unsafe {
            // Cr4::update(|f| *f |= Cr4Flags::FSGSBASE);
            CS::set_reg(GDT.1.kernel_code);
            DS::set_reg(GDT.1.kernel_data);
            ES::set_reg(GDT.1.kernel_data);
            SS::set_reg(GDT.1.kernel_data);
            FS::set_reg(GDT.1.kernel_data);
            GS::set_reg(SegmentSelector::new(0, PrivilegeLevel::Ring0));
            load_tss(GDT.1.tss);
        }
    }
}
