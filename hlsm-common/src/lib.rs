#![no_std]

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileIdentity {
    pub s_dev: u64,
    pub i_ino: u64,
    pub i_mtime_sec: i64,
}