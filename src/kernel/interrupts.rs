use super::Initialize;
use crate::kernel::gdt;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(handlers::breakpoint_handler);

        unsafe {
            idt.double_fault.set_handler_fn(handlers::double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt.page_fault.set_handler_fn(handlers::page_fault_handler);

        idt
    };
}

impl Initialize for InterruptDescriptorTable {
    fn init() {
        IDT.load();
    }
}

mod handlers {
    use crate::kernel::{hlt_loop, io::stdout::println};
    use x86_64::{
        registers::control::Cr2,
        structures::idt::{InterruptStackFrame, PageFaultErrorCode},
    };

    pub(super) extern "x86-interrupt" fn breakpoint_handler(sf: InterruptStackFrame) {
        println!("EXCEPTION: BREAKPOINT\n{:#?}", sf);
    }

    pub(super) extern "x86-interrupt" fn double_fault_handler(
        sf: InterruptStackFrame,
        _ec: u64,
    ) -> ! {
        panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", sf);
    }

    pub(super) extern "x86-interrupt" fn page_fault_handler(
        sf: InterruptStackFrame,
        ec: PageFaultErrorCode,
    ) {
        println!("EXCEPTION: PAGE FAULT");
        println!("Accessed Address: {:?}", Cr2::read());
        println!("Error Code: {:?}", ec);
        println!("{:#?}", sf);

        hlt_loop();
    }
}
