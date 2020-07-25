#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[repr(align(16))]
struct ThreadStack([u8; 65536]);

static mut THREAD_STACK: ThreadStack = ThreadStack([0; 65536]);

#[no_mangle]
pub extern "C" fn main() -> usize {
    let pipe_fd = user_lib::sys_pipe() as usize;
    unsafe {
        user_lib::sys_clone(
            thread_entry as usize,
            (&mut THREAD_STACK.0[0] as *mut u8).offset(65536) as usize,
            pipe_fd,
        );
    }
    println!("Hello world from user mode program!");
    println!("Main thread. TID = {}", user_lib::sys_gettid());

    let fd = user_lib::sys_open("hello.txt", 0) as usize;
    println!("File opened: {}", fd);

    let mut buf: [u8; 256] = [0; 256];
    let read_len = user_lib::sys_read(fd, &mut buf) as usize;
    println!("read len: {}", read_len);
    let buf_str = core::str::from_utf8(&buf[..read_len]).expect("bad utf8 bytes in file");
    println!("got string: {}", buf_str);
    user_lib::sys_write(pipe_fd, "Hello pipe!".as_bytes());
    0
}

pub extern "C" fn thread_entry(pipe_fd: usize) -> usize {
    println!("Thread started. Pipe fd = {}", pipe_fd);
    println!("Created thread. TID = {}", user_lib::sys_gettid());
    println!("Waiting on pipe.");
    loop {
        let mut buf: [u8; 64] = [0; 64];
        let n_read = user_lib::sys_read(pipe_fd, &mut buf) as usize;
        if n_read != 0 {
            println!("[thread] Got data of length {}.", n_read);
            let s = core::str::from_utf8(&buf[..n_read]).expect("thread: cannot decode pipe data");
            println!("Data from pipe: {}", s);
            break;
        }
    }
    user_lib::sys_exit(0)
}