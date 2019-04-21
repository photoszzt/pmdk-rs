#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
extern crate libc;
use libc::size_t;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const PMEMBLK_MIN_BLK: size_t = 512;

// 16MB + 4KB (minimum BTT size + mmap alignment)
#[cfg(unix)]
pub const PMEMBLK_MIN_POOL: size_t = (1 << 20) * 16 + (1 << 10) * 8;

// 16MB + 64KB (minimum BTT size + mmap alignment)
#[cfg(windows)]
pub const PMEMBLK_MIN_POOL: size_t = (1 << 20) * 16 + (1 << 10) * 64;
