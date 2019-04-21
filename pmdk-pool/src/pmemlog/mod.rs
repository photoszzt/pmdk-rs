// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.

extern crate errno;
extern crate libc;
extern crate pmdk_util;
extern crate pmemlog_sys;
extern crate rust_extra;

use self::AppendError::*;
use crate::configuration;
use errno::errno;
use libc::{c_char, c_int, c_void, mode_t, size_t};
use pmdk_util::errors::PmdkError;
use pmdk_util::is_not_null::IsNotNull;
use pmemlog_sys::*;
use rust_extra::{likely, u31, unlikely};
use std::collections::HashMap;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::sync::Arc;
use syscall_alt::constants::E::{EDEADLK, EINVAL, ENOSPC, EROFS};

include!("AppendError.rs");
include!("ForEachChunkCallback.rs");
include!("LogPool.rs");
include!("LogPoolConfiguration.rs");
include!("LogPoolDropWrapper.rs");
include!("LogPoolPathExt.rs");
include!("LogPoolsConfiguration.rs");
include!("PMEMlogpoolExt.rs");
include!("WalkCallbackResult.rs");
include!("initialise_memory_functions.rs");
