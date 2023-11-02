use std::arch::asm;

fn main() {
    unsafe {
        asm!(
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
        );
    }
}
