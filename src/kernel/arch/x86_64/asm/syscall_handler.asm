extern dispatch
global x86_64_syscall_handler

%macro push_scratch 0
    push rcx
    push rdx
    push rdi
    push rsi
    push r8
    push r9
    push r10
    push r11
%endmacro

%macro pop_scratch 0
    pop r11
    pop r10
    pop r9
    pop r8
    pop rsi
    pop rdi
    pop rdx
    pop rcx
    pop rax
%endmacro

%macro push_preserved 0
    push rbx
    push rbp
    push r12
    push r13
    push r14
    push r15
%endmacro

%macro pop_preserved 0
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbp
    pop rbx
%endmacro

x86_64_syscall_handler:
    swapgs

    push r11 ; push RFLAGS
    push rcx ; push RIP

    push rax
    push_scratch
    push_preserved

    call dispatch

    pop_scratch
    pop_preserved

    swapgs
    sysretq