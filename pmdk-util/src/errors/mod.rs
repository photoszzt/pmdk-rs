// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.

use errno::errno;
use libc::c_char;
use pmem_sys::pmem_errormsg;
use pmemblk_sys::pmemblk_errormsg;
use pmemlog_sys::pmemlog_errormsg;
use pmemobj_sys::pmemobj_errormsg;
use rust_extra::likely;
use std::error;
use std::ffi::{CStr, CString};
use std::fmt;
use std::fmt::{Display, Formatter};
use syscall_alt::constants::E::{EINVAL, ENOMEM};

include!("ErrorFunction.rs");
include!("PmdkError.rs");
include!("LastErrorMessageOnThisThreadIsInvalidError.rs");
