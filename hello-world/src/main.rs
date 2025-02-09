#![feature(link_arg_attribute)]

#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[link(kind = "link-arg", name="-nostartfiles", modifiers="+verbatim")]
extern "C" {}

#[cfg(all(target_arch = "x86_64"))]
global_asm!(
    // header
    ".text",
    ".global _start",
    "_start:",

    // print the message
    // jmp .+16 ; jump over the message
    ".byte 0xeb, 0x0e",
    // put the message right here in the code (nasty)
    ".ascii \"Hello, World!\\n\"",
    // lea rsi, [rip - 21] ; calculate pointer to message (second argument)
    ".byte 0x48, 0x8d, 0x35, 0xeb, 0xff, 0xff, 0xff",
    // mov rdx, 14 ; third argument (count)
    ".byte 0x48, 0xc7, 0xc2, 0x0e, 0x00, 0x00, 0x00",
    // mov rax, 1 ; syscall number: sys_write
    ".byte 0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00",
    // mov rdi, 1 ; first argument: stdout
    ".byte 0x48, 0xc7, 0xc7, 0x01, 0x00, 0x00, 0x00",
    // syscall
    ".byte 0x0F, 0x05",

    // exit
    // mov rax, 60 ; syscall number: sys_exit (60 = 0x3c)
    ".byte 0x48, 0xc7, 0xc0, 0x3c, 0x00, 0x00, 0x00",
    // mov rdi, 0 ; first argument: exit code
    ".byte 0x48, 0xc7, 0xc7, 0x00, 0x00, 0x00, 0x00",
    // syscall
    ".byte 0x0F, 0x05",
);
