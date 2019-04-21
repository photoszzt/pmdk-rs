#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

extern crate libc;

use libc::{c_longlong, c_void, size_t};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const PMEMOBJ_MAX_ALLOC_SIZE: size_t = 0x3FFDFFFC0;
pub const PMEMOBJ_MAX_LAYOUT: size_t = 1024;
pub const PMEMOBJ_MIN_POOL: size_t = 1024 * 1024 * 8;

pub const TX_DEFAULT_RANGE_CACHE_SIZE: c_longlong = 1 << 15;
pub const TX_DEFAULT_RANGE_CACHE_THRESHOLD: c_longlong = 1 << 12;

pub const POBJ_XALLOC_ZERO: u64 = 1 << 0;
pub const POBJ_XALLOC_NO_FLUSH: u64 = 1 << 1;
pub const POBJ_XALLOC_VALID_FLAGS: u64 = (POBJ_XALLOC_ZERO | POBJ_XALLOC_NO_FLUSH);
pub const POBJ_XADD_NO_FLUSH: u64 = 1 << 1;
pub const POBJ_XADD_VALID_FLAGS: u64 = POBJ_XADD_NO_FLUSH;

extern "C" {
    pub fn pmemobj_direct(oid: PMEMoid) -> *mut c_void;
}
