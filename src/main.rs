use std::arch::asm;

fn main() {
    let message = "Hello, World!\n";
    unsafe {
        asm!(
        "mov rax, 1",
        "mov rdi, 1",
        "syscall",
        in("rsi") message.as_ptr(),
        in("rdx") message.len(),
        );
    }
}
