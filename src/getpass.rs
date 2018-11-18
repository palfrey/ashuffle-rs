use libc;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;

pub unsafe fn as_getpass(
    mut in_stream: *mut libc::FILE,
    mut out_stream: *mut libc::FILE,
    mut prompt: *const libc::c_char,
) -> *mut libc::c_char {
    if libc::fwrite(
        prompt as *const libc::c_void,
        libc::strlen(prompt),
        1i32 as libc::size_t,
        out_stream,
    ) != 1
    {
        libc::perror(b"getpass (fwrite)\x00" as *const u8 as *const libc::c_char);
        ::std::process::exit(1i32);
    } else {
        set_echo(out_stream, 0 != 0i32, 0 != 1i32);
        let mut result = 0 as *mut libc::c_char;
        let mut result_size = 0i32 as libc::size_t;
        let mut result_len: libc::ssize_t = libc::getline(&mut result, &mut result_size, in_stream);
        if result_len < 0 {
            libc::perror(b"getline (getpass)\x00" as *const u8 as *const libc::c_char);
            ::std::process::exit(1i32);
        } else {
            set_echo(out_stream, 0 != 1i32, 0 != 1i32);
            return result;
        }
    };
}
unsafe fn set_echo(
    mut stream: *mut libc::FILE,
    mut echo_state: bool,
    mut echo_nl_state: bool,
) {
    let mut flags = libc::termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_cc: [0; 20],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    let mut res: libc::c_int = libc::tcgetattr(libc::fileno(stream), &mut flags);
    if res != 0i32 {
        libc::perror(b"set_echo (tcgetattr)\x00" as *const u8 as *const libc::c_char);
        ::std::process::exit(1i32);
    } else {
        if echo_state {
            flags.c_lflag |= libc::ECHO
        } else {
            flags.c_lflag &= libc::ECHO
        }
        if echo_nl_state {
            flags.c_lflag |= libc::ECHONL
        } else {
            flags.c_lflag &= !libc::ECHONL
        }
        res = libc::tcsetattr(libc::fileno(stream), 0i32, &mut flags);
        if res != 0i32 {
            libc::perror(b"set_echo (tcsetattr)\x00" as *const u8 as *const libc::c_char);
            ::std::process::exit(1i32);
        } else {
            return;
        }
    };
}
