use super::{
    gdt, hlt_loop,
    io::{
        keyboard,
        vga::{print, println},
    },
    Initialize,
};
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::{
    instructions::port::Port,
    registers::control::Cr2,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode},
};

pub fn init() {
    InterruptDescriptorTable::init();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
            idt.page_fault
                .set_handler_fn(page_fault_handler)
                .set_stack_index(gdt::PAGE_FAULT_IST_INDEX);
        }

        idt[Into::<usize>::into(InterruptIndex::Timer)].set_handler_fn(timer_interrupt_handler);
        idt[Into::<usize>::into(InterruptIndex::Keyboard)]
            .set_handler_fn(keyboard_interrupt_handler);

        idt
    };
}

const PIC1_OFFSET: u8 = 32;
const PIC2_OFFSET: u8 = PIC1_OFFSET + 8;

static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC1_OFFSET, PIC2_OFFSET) });

impl Initialize for InterruptDescriptorTable {
    fn init() {
        IDT.load();
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum InterruptIndex {
    Timer = PIC1_OFFSET,
    Keyboard,
}

impl Into<u8> for InterruptIndex {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Into<usize> for InterruptIndex {
    fn into(self) -> usize {
        (self as u8).into()
    }
}

extern "x86-interrupt" fn breakpoint_handler(sf: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\nStack Frame: {:#?}", sf);
}

extern "x86-interrupt" fn double_fault_handler(sf: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\nStack Frame: {:#?}", sf);
}

extern "x86-interrupt" fn page_fault_handler(
    sf: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("Stack Frame: {:#?}", sf);

    hlt_loop();
}

extern "x86-interrupt" fn timer_interrupt_handler(_sf: InterruptStackFrame) {
    print!(".");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(Into::<u8>::into(InterruptIndex::Timer));
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_sf: InterruptStackFrame) {
    if let Some(ref mut kbd) = *keyboard::KEYBOARD.lock() {
        let scancode = keyboard::read_scancode();

        if let Ok(Some(key_event)) = kbd.add_byte(scancode) {
            if let Some(key) = kbd.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => print!("{}", character),
                    DecodedKey::RawKey(key) => print!("{:?}", key),
                }
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(Into::<u8>::into(InterruptIndex::Keyboard));
    }
}
