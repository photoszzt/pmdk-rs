// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.

use crate::configuration;
use errno::errno;
use libc::{
    c_char, c_int, c_longlong, c_void, mode_t, size_t, EAGAIN, EBUSY, ECANCELED, EDEADLK, EINVAL,
    ENOTRECOVERABLE, EOWNERDEAD, EPERM, ETIMEDOUT,
};
use pmdk_util::errors::PmdkError;
use pmdk_util::is_not_null::IsNotNull;
use pmemobj_sys::*;
use rust_extra::{likely, unlikely};
use std::any::Any;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ffi::CString;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::mem::{replace, size_of, uninitialized};
use std::ops::{Deref, DerefMut};
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use std::path::Path;
use std::ptr::{null, null_mut};
use std::sync::Arc;

include!("ConditionVariable.rs");
include!("ConditionVariableMutexLockablePersistable.rs");
include!("initialise_memory_functions.rs");
include!("Initializable.rs");
include!("ListEntryPersistable.rs");
include!("MutexLock.rs");
include!("MutexLockablePersistable.rs");
include!("MutexUnlock.rs");
include!("ObjectPool.rs");
include!("ObjectPoolConfiguration.rs");
include!("ObjectPoolControl.rs");
include!("ObjectPoolDropWrapper.rs");
include!("ObjectPoolPersistOnDrop.rs");
include!("ObjectPoolPathExt.rs");
include!("ObjectPoolsConfiguration.rs");
include!("OID.rs");
include!("PersistentObject.rs");
include!("Persistable.rs");
include!("PersistentCircularDoublyLinkedListEntry.rs");
include!("PersistentCircularDoublyLinkedListHead.rs");
include!("PMEMobjpoolExt.rs");
include!("PMEMoidIterator.rs");
include!("ReadLockUnlock.rs");
include!("ReadWriteLock.rs");
include!("ReadWriteLockablePersistable.rs");
include!("TypeNumber.rs");
include!("Transaction.rs");
include!("WriteLockUnlock.rs");
include!("Zero.rs");
