//! 系统调用

pub const STDIN: usize = 0;
pub const STDOUT: usize = 1;

const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_CLONE: usize = 256;
const SYSCALL_GETTID: usize = 257;
const SYSCALL_OPEN: usize = 258;
const SYSCALL_PIPE: usize = 259;

/// 将参数放在对应寄存器中，并执行 `ecall`
fn syscall(id: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    // 返回值
    let mut ret;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (id)
            : "memory"      // 如果汇编可能改变内存，则需要加入 memory 选项
            : "volatile"); // 防止编译器做激进的优化（如调换指令顺序等破坏 SBI 调用行为的优化）
    }
    ret
}

/// 读取字符
pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    loop {
        let ret = syscall(
            SYSCALL_READ,
            fd,
            buffer as *const [u8] as *const u8 as usize,
            buffer.len(),
        );
        if ret > 0 {
            return ret;
        }
    }
}

/// 打印字符串
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(
        SYSCALL_WRITE,
        fd,
        buffer as *const [u8] as *const u8 as usize,
        buffer.len(),
    )
}

pub unsafe fn sys_clone(pc: usize, sp: usize, user_context: usize) -> isize {
    syscall(
        SYSCALL_CLONE,
        pc,
        sp,
        user_context,
    )
}

pub fn sys_gettid() -> isize {
    syscall(SYSCALL_GETTID, 0, 0, 0)
}

/// 退出并返回数值
pub fn sys_exit(code: isize) -> ! {
    syscall(SYSCALL_EXIT, code as usize, 0, 0);
    unreachable!()
}

pub fn sys_open(path: &str, mode: u32) -> isize {
    let path = path.as_bytes();
    syscall(
        SYSCALL_OPEN,
        path.as_ptr() as _,
        path.len(),
        mode as _,
    )
}

pub fn sys_pipe() -> isize {
    syscall(
        SYSCALL_PIPE,
        0,
        0,
        0,
    )
}