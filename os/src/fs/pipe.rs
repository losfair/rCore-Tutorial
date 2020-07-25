use rcore_fs::vfs::*;
use core::any::Any;
use alloc::collections::LinkedList;
use alloc::collections::vec_deque::VecDeque;
use spin::Mutex;
use alloc::sync::Arc;

pub struct Pipe {
    buffer: Mutex<VecDeque<u8>>,
}

impl Pipe {
    pub fn new() -> Arc<Self> {
        Arc::new(Pipe {
            buffer: Mutex::new(VecDeque::new()),
        })
    }
}
impl INode for Pipe {
    fn read_at(&self, _offset: usize, buf: &mut [u8]) -> Result<usize> {
        let mut buffer = self.buffer.lock();
        for i in 0..buf.len() {
            buf[i] = match buffer.pop_front() {
                Some(x) => x,
                None => {
                    return Ok(i);
                }
            };
        }
        Ok(buf.len())
    }

    fn write_at(&self, _offset: usize, buf: &[u8]) -> Result<usize> {
        let mut buffer = self.buffer.lock();
        for i in 0..buf.len() {
            buffer.push_back(buf[i]);
        }
        Ok(buf.len())
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn poll(&self) -> Result<PollStatus> {
        unimplemented!("Pipe::poll");
    }
}
