use x86_64::{
    registers::{
        model_specific::{Efer, EferFlags, LStar, SFMask, Star},
        rflags::{self, RFlags},
    },
    VirtAddr,
};

extern "C" {
    fn x86_64_syscall_handler();
}

pub(crate) fn init() {
    unsafe {
        Efer::update(|f| f.insert(EferFlags::SYSTEM_CALL_EXTENSIONS));
        Star::write_raw(35, 8);
    }

    SFMask::write(rflags::read() ^ RFlags::INTERRUPT_FLAG);
    LStar::write(VirtAddr::new(x86_64_syscall_handler as u64));
}
