use super::{
    gdt, hlt_loop,
    io::{
        keyboard,
        vga::{exception, print, println},
    },
    process::Registers,
    syscall, Initialize,
};
use core::arch::asm;
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};
use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::{
    instructions::{hlt, port::Port},
    registers::control::Cr2,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode},
    PrivilegeLevel,
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
            idt[0x80]
                .set_handler_fn(core::mem::transmute(wrapped_syscall_handler as *mut fn()))
                .set_privilege_level(PrivilegeLevel::Ring3);
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
    exception!("BREAKPOINT\nStack Frame: {:#?}", sf);
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

extern "x86-interrupt" fn timer_interrupt_handler(_sf: InterruptStackFrame) {
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
        PICS.lock()
            .notify_end_of_interrupt(Into::<u8>::into(InterruptIndex::Keyboard));
    }
}

extern "sysv64" fn syscall_handler(sf: &mut InterruptStackFrame, registers: &mut Registers) {
    let (id, arg1, arg2, arg3, arg4) = (
        registers.rax,
        registers.rdi,
        registers.rsi,
        registers.rdx,
        registers.r8,
    );

    let res = syscall::dispatch(id, arg1, arg2, arg3, arg4);

    registers.rax = res;

    unsafe { PICS.lock().notify_end_of_interrupt(0x80) }
}

macro wrap($fn: ident => $w:ident) {
    #[naked]
    unsafe extern "sysv64" fn $w() {
        asm!(
            "push rax",
            "push rcx",
            "push rdx",
            "push rsi",
            "push rdi",
            "push r8",
            "push r9",
            "push r10",
            "push r11",
            "mov rsi, rsp",
            "mov rdi, rsp",
            "add rdi, 9 * 8",
            "call {}",
            "pop r11",
            "pop r10",
            "pop r9",
            "pop r8",
            "pop rdi",
            "pop rsi",
            "pop rdx",
            "pop rcx",
            "pop rax",
            "iretq",
            sym $fn,
            options(noreturn)
        );
    }
}

wrap!(syscall_handler => wrapped_syscall_handler);
