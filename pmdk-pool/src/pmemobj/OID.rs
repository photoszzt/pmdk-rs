// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.

/// Every persistable object has an OID, Object Identifier, which is an unique identifier in persistent storage. Object identifiers are not unique across program lifetimes, ie not unique across time.
/// Ordinarily there is no need to use this interface; prefer using the wrapper type PersistentObject.
pub trait OID {
    /// Is this instance equivalent to null?
    #[inline(always)]
    fn is_null(&self) -> bool;

    /// Is this instance not equivalent to null?
    #[inline(always)]
    fn is_not_null(&self) -> bool;

    /// Comparision.
    #[inline(always)]
    fn equals(&self, other: &Self) -> bool;

    /// Can be NULL, but only if is_null() is true.
    #[inline(always)]
    fn object_pool(&self) -> *mut PMEMobjpool;

    /// Actual allocated size.
    #[inline(always)]
    fn allocated_useful_size(&self) -> size_t;

    /// type number; unique for each type (struct). Root type is number 0.
    #[inline(always)]
    fn type_number(&self) -> TypeNumber;

    /// Can be NULL, but only if is_null() is true.
    #[inline(always)]
    fn address(&self) -> *mut c_void;
}

impl OID for PMEMoid {
    #[inline(always)]
    fn is_null(&self) -> bool {
        OID_IS_NULL(self)
    }

    #[inline(always)]
    fn is_not_null(&self) -> bool {
        !self.is_null()
    }

    #[inline(always)]
    fn equals(&self, other: &Self) -> bool {
        OID_EQUALS(self, other)
    }

    #[inline(always)]
    fn object_pool(&self) -> *mut PMEMobjpool {
        unsafe { pmemobj_pool_by_oid(*self) }
    }

    #[inline(always)]
    fn allocated_useful_size(&self) -> size_t {
        unsafe { pmemobj_alloc_usable_size(*self) }
    }

    #[inline(always)]
    fn type_number(&self) -> TypeNumber {
        unsafe { pmemobj_type_num(*self) }
    }

    #[inline(always)]
    fn address(&self) -> *mut c_void {
        unsafe { pmemobj_direct(*self) }
    }
}

#[allow(non_snake_case)]
#[inline(always)]
fn OID_IS_NULL(o: &PMEMoid) -> bool {
    o.off == 0
}

#[allow(non_snake_case)]
#[inline(always)]
fn OID_EQUALS(lhs: &PMEMoid, rhs: &PMEMoid) -> bool {
    lhs.off == rhs.off && lhs.pool_uuid_lo == rhs.pool_uuid_lo
}
