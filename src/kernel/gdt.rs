use super::Initialize;
use lazy_static::lazy_static;
use x86_64::{
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
    VirtAddr,
};

pub(crate) fn init() {
    GlobalDescriptorTable::init();
}

const STACK_SIZE: usize = 1024 * 8;
pub(super) const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub(super) const PAGE_FAULT_IST_INDEX: u16 = 1;

macro ist($tss:expr, $($idx:ident)+) {
    $(
        $tss.interrupt_stack_table[$idx as usize] = {
            VirtAddr::from_ptr(&STACK) + STACK_SIZE
        };
    )+
}

lazy_static! {
    static ref STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        ist!(tss, DOUBLE_FAULT_IST_INDEX PAGE_FAULT_IST_INDEX);
        tss
    };
    pub(crate) static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();

        let code = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss = gdt.add_entry(Descriptor::tss_segment(&TSS));
        let data = gdt.add_entry(Descriptor::kernel_data_segment());
        let user_code = gdt.add_entry(Descriptor::user_code_segment());
        let user_data = gdt.add_entry(Descriptor::user_data_segment());

        (
            gdt,
            Selectors {
                tss,
                code,
                data,
                user_code,
                user_data,
            },
        )
    };
}

pub(crate) struct Selectors {
    tss: SegmentSelector,
    code: SegmentSelector,
    data: SegmentSelector,
    pub user_code: SegmentSelector,
    pub user_data: SegmentSelector,
}

impl Initialize for GlobalDescriptorTable {
    fn init() {
        use x86_64::instructions::{
            segmentation::{Segment, CS, DS},
            tables::load_tss,
        };

        GDT.0.load();

        unsafe {
            CS::set_reg(GDT.1.code);
            DS::set_reg(GDT.1.data);
            load_tss(GDT.1.tss);
        }
    }
}
