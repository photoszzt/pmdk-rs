// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.
extern crate errno;
#[macro_use]
extern crate pmdk_util;
extern crate pmem_sys;
extern crate syscall_alt;
#[macro_use]
extern crate bitflags;

use self::file_backed_memory::*;
use errno::errno;
use libc::{c_char, c_void, mode_t};
use pmdk_util::errors::PmdkError;
use pmdk_util::is_not_null::IsNotNull;
use pmem_sys::*;
use rust_extra::{likely, unlikely};
use std::cell::Cell;
use std::marker::PhantomData;
use std::mem::uninitialized;
use std::path::Path;
use std::ptr::{copy, copy_nonoverlapping, write_bytes};
use std::sync::Arc;
use syscall_alt::constants::E::{EBUSY, EINVAL, ENOMEM};

/// Different kinds of file-backed persistent memory.
pub mod file_backed_memory;

/// Different kinds of persistence and flushing for drop of file-backed persistent memory references.
pub mod persist_on_drop;

/// Different implementations of persistence.
pub mod persistence;

include!("c_voidExt.rs");
include!("c_voidMutExt.rs");
include!("has_hardware_drain_instruction.rs");
