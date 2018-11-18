// #![feature(libc)]
#![feature(extern_types)]
// #![feature(asm)]
// #![feature(ptr_wrapping_offset_from)]
// #![feature(const_slice_as_ptr)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(unused_mut)]

extern crate libc;
pub mod args;
pub mod ashuffle;
pub mod getpass;
pub mod list;
pub mod rule;
pub mod shuffle;
pub mod mpd;
pub mod streams;

fn main() { ashuffle::main() }
