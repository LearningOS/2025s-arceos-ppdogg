pub fn sys_mmap(
    _addr: *mut usize,
    _length: usize,
    _prot: i32,
    _flags: i32,
    _fd: i32,
    _offset: isize,
) -> isize {
    0
}