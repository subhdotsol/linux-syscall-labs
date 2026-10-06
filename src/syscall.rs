use core::arch::asm;

/// Linux AArch64 syscall
///
/// x8 = syscall number
/// x0 = arg0
/// x1 = arg1
/// x2 = arg2
/// x0 = return value
/// svc #0 = enter kernel
#[inline(always)]
pub unsafe fn syscall3(number: u64, arg0: u64, arg1: u64, arg2: u64) -> u64 {
    let ret: u64;

    unsafe {
        asm!(
            "svc #0",
            in("x8") number,
            in("x0") arg0,
            in("x1") arg1,
            in("x2") arg2,
            lateout("x0") ret,
            lateout("x8") _,
        );
    }

    ret
}
