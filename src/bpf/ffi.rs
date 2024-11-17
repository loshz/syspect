use std::ptr;

use libc::{c_uchar, pid_t};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct Process {
    pub pname: [c_uchar; 16],
    pub pid: pid_t,
}

impl From<Vec<u8>> for Process {
    fn from(v: Vec<u8>) -> Self {
        let p: Process = unsafe { ptr::read(v.as_ptr() as *const _) };
        p
    }
}
