use super::{stdout::println, Initialize};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

macro handlers($inst:ident, $($ie:ident),*) {
    use core::concat_idents;

    $(
        $inst.$ie.set_handler_fn(concat_idents!($ie, _handler));
    )*
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        handlers!(idt, breakpoint, double_fault);
        idt
    };
}

impl Initialize for InterruptDescriptorTable {
    fn init() {
        IDT.load();
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _ec: u64) -> ! {
    panic!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
