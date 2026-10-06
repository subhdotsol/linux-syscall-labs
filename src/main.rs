// #![no_std]
// #![no_main]

// use core::panic::PanicInfo;

// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {}
// }

// #[unsafe(no_mangle)]
// pub extern "C" fn _start() -> ! {
//     let message = b"Hello from no_std Rust!\n";

//     unsafe {
//         syscall_write(1, message.as_ptr(), message.len());
//         syscall_exit(0);
//     }
// }

// unsafe fn syscall_write(fd: usize, buffer: *const u8, length: usize) {
//     unsafe {
//         core::arch::asm!(
//             "svc 0",
//             in("x8") 64usize,
//             in("x0") fd,
//             in("x1") buffer,
//             in("x2") length,
//             lateout("x0") _,
//         );
//     }
// }

// unsafe fn syscall_exit(code: usize) -> ! {
//     unsafe {
//         core::arch::asm!(
//             "svc 0",
//             in("x8") 93usize,
//             in("x0") code,
//             options(noreturn),
//         );
//     }
// }

#![no_std]
#![no_main]

mod syscall;

use core::panic::PanicInfo;
use syscall::syscall3;

const SYS_WRITE: u64 = 64;
const SYS_EXIT: u64 = 93;

const STDOUT: u64 = 1;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn print(msg: &[u8]) {
    unsafe {
        syscall3(SYS_WRITE, STDOUT, msg.as_ptr() as u64, msg.len() as u64);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    print(b"Hello, alessandro!\n");

    unsafe {
        syscall3(SYS_EXIT, 0, 0, 0);
    }

    unreachable!()
}
