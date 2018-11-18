#![allow(non_camel_case_types)]

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
