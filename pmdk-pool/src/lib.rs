#![feature(custom_attribute)]
#![feature(specialization)]
#![feature(thread_local)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate pmdk_util;
extern crate errno;
extern crate libc;
extern crate pmemblk_sys;
extern crate pmemlog_sys;
extern crate pmemobj_sys;
#[macro_use]
extern crate quick_error;
extern crate rust_extra;

pub mod configuration;
pub mod pmemblk;
pub mod pmemlog;
pub mod pmemobj;
pub mod pools;
