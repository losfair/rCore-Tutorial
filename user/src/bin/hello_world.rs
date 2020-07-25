#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[repr(align(16))]
struct ThreadStack([u8; 65536]);

static mut THREAD_STACK: ThreadStack = ThreadStack([0; 65536]);

#[no_mangle]
pub extern "C" fn main() -> usize {
    unsafe {
        user_lib::sys_clone(
            thread_entry as usize,
            (&mut THREAD_STACK.0[0] as *mut u8).offset(65536) as usize,
            42,
        );
    }
    println!("Hello world from user mode program!");
    0
}

pub extern "C" fn thread_entry(value: usize) -> usize {
    println!("Thread started. Arg = {}", value);
    user_lib::sys_exit(0)
}