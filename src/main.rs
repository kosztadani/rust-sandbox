use std::arch::asm;

fn main() {
    let message = "Hello, World!\n";
    unsafe {
        asm!(
        // mov rax, 1
        ".byte 0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00",
        // mov rdi, 1
        ".byte 0x48, 0xc7, 0xc7, 0x01, 0x00, 0x00, 0x00",
        // syscall
        ".byte 0x0F, 0x05",
        in("rsi") message.as_ptr(),
        in("rdx") message.len(),
        );
    }
}
