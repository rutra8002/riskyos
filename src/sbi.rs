

mod ext {
    pub const CONSOLE: usize = 0x4442434E; // DBCM
    pub const SYSTEM: usize = 0x53525354; //SRST
}

mod func {
    pub const WRITE_BYTE: usize = 0x02;
    pub const SHUTDOWN: usize = 0x00;
}

fn call(ext: usize, func: usize, arg0: usize) -> usize {
    let ret;
    unsafe {
        core::arch::asm!(
        "ecall",
        in("a0") arg0,
        in("a6") func,
        in("a7") ext,
        lateout("a0") ret,
        options(nostack)
        );
    }
    ret
}

pub fn putchar(c: u8) {
    call(ext::CONSOLE, func::WRITE_BYTE, c as usize);
}

pub fn print(s: &str) {
    for b in s.bytes() {
        putchar(b);
    }
}

pub fn shutdown() -> ! {
    call(ext::SYSTEM, func::SHUTDOWN, 0);
    loop {}
}