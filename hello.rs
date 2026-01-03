
#![no_std]
#![no_main]

use core::arch::asm;

/* --------------------------------
   strlen manual
   -------------------------------- */
fn len(mut s: *const u8) -> usize {
    let mut n: usize = 0;
    unsafe {
        while *s != 0 {
            s = s.add(1);
            n += 1;
        }
    }
    n
}

/* --------------------------------
   syscall write (int 0x80)
   eax = 4
   ebx = fd
   ecx = buffer
   edx = size
   -------------------------------- */
unsafe fn sys_write(fd: u32, buf: *const u8, size: usize) {
    asm!(
        "int 0x80",
        in("eax") 4u32,
        in("ebx") fd,
        in("ecx") buf,
        in("edx") size,
        options(nostack, preserves_flags),
    );
}

/* --------------------------------
   sputs
   -------------------------------- */
fn sputs(s: *const u8) {
    unsafe {
        sys_write(1, s, len(s));
    }
}

/* --------------------------------
   syscall exit (int 0x80)
   eax = 1
   ebx = status
   -------------------------------- */
unsafe fn exits(code: u32) -> ! {
    asm!(
        "int 0x80",
        in("eax") 1u32,
        in("ebx") code,
        options(noreturn),
    );
}

/* --------------------------------
   Entry point real (_start)
   -------------------------------- */
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let msg = b"Hello from rust -nostdlib i386\n\0";
    sputs(msg.as_ptr());
    unsafe { exits(0) }
}
