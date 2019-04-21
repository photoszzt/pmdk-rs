#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
extern crate libc;
use libc::size_t;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const PMEMLOG_MIN_POOL: size_t = 1024 * 1024 * 2;
