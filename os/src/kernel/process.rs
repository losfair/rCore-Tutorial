//! 进程相关的内核功能

use super::*;

pub(super) fn sys_exit(code: usize) -> SyscallResult {
    println!(
        "thread {} exit with code {}",
        PROCESSOR.lock().current_thread().id,
        code
    );
    SyscallResult::Kill
}

pub(super) fn sys_clone(context: &mut Context, new_pc: usize, new_sp: usize, user_context: usize) -> SyscallResult {
    let result = PROCESSOR.lock().current_thread().fork(context, new_pc, new_sp, user_context);
    match result {
        Ok(t) => {
            PROCESSOR.lock().add_thread(t);
            SyscallResult::Proceed(0)
        }
        Err(_) => {
            println!("sys_clone failed");
            SyscallResult::Kill
        }
    }
}