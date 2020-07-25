//! 文件相关的内核功能

use super::*;
use core::slice::{from_raw_parts, from_raw_parts_mut};
use crate::fs::{Pipe, ROOT_INODE};

/// 从指定的文件中读取字符
///
/// 如果缓冲区暂无数据，返回 0；出现错误返回 -1
pub(super) fn sys_read(fd: usize, buffer: *mut u8, size: usize) -> SyscallResult {
    // 从进程中获取 inode
    let process = PROCESSOR.lock().current_thread().process.clone();
    if let Some(inode) = process.inner().descriptors.get(fd) {
        // 从系统调用传入的参数生成缓冲区
        let buffer = unsafe { from_raw_parts_mut(buffer, size) };
        // 尝试读取
        if let Ok(ret) = inode.read_at(0, buffer) {
            let ret = ret as isize;
            if ret > 0 {
                return SyscallResult::Proceed(ret);
            }
            if ret == 0 {
                return SyscallResult::Park(ret);
            }
        }
    }
    SyscallResult::Proceed(-1)
}

/// 将字符写入指定的文件
pub(super) fn sys_write(fd: usize, buffer: *mut u8, size: usize) -> SyscallResult {
    // 从进程中获取 inode
    let process = PROCESSOR.lock().current_thread().process.clone();
    if let Some(inode) = process.inner().descriptors.get(fd) {
        // 从系统调用传入的参数生成缓冲区
        let buffer = unsafe { from_raw_parts_mut(buffer, size) };
        // 尝试写入
        if let Ok(ret) = inode.write_at(0, buffer) {
            let ret = ret as isize;
            if ret >= 0 {
                return SyscallResult::Proceed(ret);
            }
        }
    }
    SyscallResult::Proceed(-1)
}

pub(super) fn sys_open(path_ptr: *const u8, path_len: usize, _mode: u32) -> SyscallResult {
    // Dereferencing user memory. VERY UNSAFE but anyway this is just a lab :)
    let path = unsafe {
        from_raw_parts(path_ptr, path_len)
    };
    let path = match core::str::from_utf8(path) {
        Ok(x) => x,
        Err(_) => return SyscallResult::Proceed(-1),
    };
    let handle = if let Ok(x) = ROOT_INODE.find(path) {
        x
    } else {
        return SyscallResult::Proceed(-2);
    };

    let fd;

    {
        let processor = PROCESSOR.lock();
        let process = &processor.current_thread().process;
        let mut inner = process.inner();
        fd = inner.descriptors.len();
        inner.descriptors.push(handle);
    }

    SyscallResult::Proceed(fd as _)
}

pub(super) fn sys_pipe() -> SyscallResult {
    let pipe = Pipe::new();
    let fd;

    {
        let processor = PROCESSOR.lock();
        let process = &processor.current_thread().process;
        let mut inner = process.inner();
        fd = inner.descriptors.len();
        inner.descriptors.push(pipe);
    }

    SyscallResult::Proceed(fd as _)
}