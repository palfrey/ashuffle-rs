use libc;

extern {
    pub fn stdin_file() -> *mut libc::FILE;
    pub fn errno_val() -> libc::c_int;
}