#![deny(warnings, unused_mut)]
#![allow(non_camel_case_types)]

extern crate libc;
extern crate easycurses;

mod args;
mod ashuffle;
mod getpass;
mod list;
mod rule;
mod shuffle;
mod mpd;
mod streams;

pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(ashuffle::main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}