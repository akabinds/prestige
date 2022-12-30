use super::{gdt, hlt_loop};
use crate::kernel::{
    io::{
        console, exception,
        keyboard::{self, ALT, CTRL, SHIFT},
        println,
        serial::SERIAL,
    },
    syscall, Initialize,
};
use core::sync::atomic::Ordering;
use lazy_static::lazy_static;
use pc_keyboard::{DecodedKey, KeyCode, KeyState};
use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::{
    instructions::interrupts as x86_64cint, // x86_64 crate interrupts
    registers::control::Cr2,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode},
};

pub(crate) fn init() {
    InterruptDescriptorTable::init();
    unsafe { PICS.lock().initialize() };
    x86_64cint::enable();
}

pub(crate) fn halt() {
    let disabled = x86_64cint::are_enabled();
    x86_64cint::enable_and_hlt();

    if disabled {
        x86_64cint::enable();
    }
}

fn irq_idx(n: u8) -> usize {
    (PIC1_OFFSET + n) as usize
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.divide_error.set_handler_fn(div_by_zero_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.general_protection_fault
            .set_handler_fn(general_protection_fault_handler);
        idt.stack_segment_fault
            .set_handler_fn(stack_segment_fault_handler);
        idt.segment_not_present
            .set_handler_fn(segment_not_present_handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt[irq_idx(0)].set_handler_fn(timer_interrupt_handler);
        idt[irq_idx(1)].set_handler_fn(keyboard_interrupt_handler);
        idt[irq_idx(4)].set_handler_fn(com1_serial_interrupt_handler);

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

extern "x86-interrupt" fn breakpoint_handler(sf: InterruptStackFrame) {
    exception!("BREAKPOINT\nStack Frame: {:#?}", sf);
}

extern "x86-interrupt" fn div_by_zero_handler(sf: InterruptStackFrame) {
    exception!("DIVISION BY ZERO\nStack Frame: {:#?}", sf);
}

extern "x86-interrupt" fn double_fault_handler(sf: InterruptStackFrame, _ec: u64) -> ! {
    exception!("DOUBLE FAULT\nStack Frame: {:#?}", sf);

    hlt_loop();
}

extern "x86-interrupt" fn page_fault_handler(sf: InterruptStackFrame, ec: PageFaultErrorCode) {
    exception!("PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", ec);
    println!("Stack Frame: {:#?}", sf);

    hlt_loop();
}

extern "x86-interrupt" fn general_protection_fault_handler(sf: InterruptStackFrame, ec: u64) {
    exception!("GENERAL PROTECTION FAULT");
    println!("Error Code: {:?}", ec);
    println!("Stack Frame: {:#?}", sf);

    hlt_loop();
}

extern "x86-interrupt" fn stack_segment_fault_handler(sf: InterruptStackFrame, ec: u64) {
    exception!("STACK SEGMENT FAULT");
    println!("Error Code: {:?}", ec);
    println!("Stack Frame: {:#?}", sf);

    hlt_loop();
}

extern "x86-interrupt" fn segment_not_present_handler(sf: InterruptStackFrame, ec: u64) {
    exception!("SEGMENT NOT PRESENT");
    println!("Error Code: {:?}", ec);
    println!("Stack Frame: {:#?}", sf);

    hlt_loop();
}

extern "x86-interrupt" fn timer_interrupt_handler(_sf: InterruptStackFrame) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(irq_idx(0) as u8);
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_sf: InterruptStackFrame) {
    if let Some(ref mut kbd) = *keyboard::KEYBOARD.lock() {
        let scancode = keyboard::read_scancode();

        if let Ok(Some(key_event)) = kbd.add_byte(scancode) {
            match key_event.code {
                KeyCode::AltLeft | KeyCode::AltRight => {
                    ALT.store(key_event.state == KeyState::Down, Ordering::Relaxed)
                }
                KeyCode::ShiftLeft | KeyCode::ShiftRight => {
                    SHIFT.store(key_event.state == KeyState::Down, Ordering::Relaxed)
                }
                KeyCode::ControlLeft | KeyCode::ControlRight => {
                    CTRL.store(key_event.state == KeyState::Down, Ordering::Relaxed)
                }
                _ => {}
            }

            let is_alt = ALT.load(Ordering::Relaxed);
            let is_ctrl = CTRL.load(Ordering::Relaxed);
            let is_shift = SHIFT.load(Ordering::Relaxed);

            if let Some(key) = kbd.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode('\t') if is_shift => keyboard::send_csi('Z'),
                    DecodedKey::Unicode('\u{7f}') if is_ctrl && is_alt => syscall::reboot(),
                    DecodedKey::Unicode(c) => keyboard::send_key(c),
                    DecodedKey::RawKey(KeyCode::ArrowUp) => keyboard::send_csi('A'),
                    DecodedKey::RawKey(KeyCode::ArrowDown) => keyboard::send_csi('B'),
                    DecodedKey::RawKey(KeyCode::ArrowRight) => keyboard::send_csi('C'),
                    DecodedKey::RawKey(KeyCode::ArrowLeft) => keyboard::send_csi('D'),
                    _ => {}
                }
            }
        }
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(irq_idx(1) as u8);
    }
}

extern "x86-interrupt" fn com1_serial_interrupt_handler(_sf: InterruptStackFrame) {
    let byte = SERIAL.lock().read_byte();
    let key = match byte as char {
        '\r' => '\n',
        '\x7F' => '\x08',
        c => c,
    };

    console::handle_key_inp(key);

    unsafe {
        PICS.lock().notify_end_of_interrupt(irq_idx(4) as u8);
    }
}
