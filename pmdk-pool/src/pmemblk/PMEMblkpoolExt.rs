// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.

/// Extension trait to make it easier to work with PMEMblkpool.
trait PMEMblkpoolExt {
    /// Close the block pool.
    /// self can not be null.
    #[inline(always)]
    fn close(self);

    /// Size of blocks in the block pool.
    /// self can not be null.
    #[inline(always)]
    fn block_size(self) -> usize;

    /// How many blocks are available (free) in the block pool?
    /// self can not be null.
    #[inline(always)]
    fn number_of_blocks_available_in_block_pool(self) -> usize;

    /// Read from a block.
    /// Returns false if the block has previously had its error condition set (see `set_error()`).
    /// to can not be null.
    /// self can not be null.
    #[inline(always)]
    fn read_from(self, to: *mut c_void, zero_based_block_index: usize) -> bool;

    /// Write to a block.
    /// from can not be null.
    /// self can not be null.
    #[inline(always)]
    fn write_to(self, from: *const c_void, zero_based_block_index: usize);

    /// Set a block to all zeros.
    /// self can not be null.
    #[inline(always)]
    fn set_zero(self, zero_based_block_index: usize);

    /// Set a block to being in an error state (ie set its error condition).
    /// self can not be null.
    #[inline(always)]
    fn set_error(self, zero_based_block_index: usize);
}

macro_rules! debug_assert_self_is_not_null {
    ($self: ident) => {
        debug_assert!($self.is_not_null(), "PMEMblkpool (pbp) can not be null");
    };
}

impl PMEMblkpoolExt for *mut PMEMblkpool {
    #[inline(always)]
    fn close(self) {
        debug_assert_self_is_not_null!(self);

        unsafe { pmemblk_close(self) }
    }

    #[inline(always)]
    fn block_size(self) -> usize {
        debug_assert_self_is_not_null!(self);

        unsafe { pmemblk_bsize(self) }
    }

    #[inline(always)]
    fn number_of_blocks_available_in_block_pool(self) -> usize {
        debug_assert_self_is_not_null!(self);

        unsafe { pmemblk_nblock(self) }
    }

    #[inline(always)]
    fn read_from(self, to: *mut c_void, zero_based_block_index: usize) -> bool {
        debug_assert_self_is_not_null!(self);
        debug_assert!(to.is_not_null(), "to can not be null");
        debug_assert!(
            zero_based_block_index < self.number_of_blocks_available_in_block_pool(),
            "zero_based_block_index '{}' exceeds number_of_blocks_available_in_block_pool '{}'",
            zero_based_block_index,
            self.number_of_blocks_available_in_block_pool()
        );

        let result = unsafe { pmemblk_read(self, to, zero_based_block_index as c_longlong) };
        if likely(result == 0) {
            return true;
        } else if unlikely(result != -1) {
            panic!(
                "pmemblk_read() return a value which wasn't -1 or 0, but '{}'",
                result
            );
        } else {
            let os_error_number = errno().0;
            if likely(os_error_number == EIO) {
                false
            } else {
                PmdkError::block_panic("pmemblk_read")
            }
        }
    }

    #[inline(always)]
    fn write_to(self, from: *const c_void, zero_based_block_index: usize) {
        debug_assert_self_is_not_null!(self);
        debug_assert!(from.is_not_null(), "from can not be null");
        debug_assert!(
            zero_based_block_index < self.number_of_blocks_available_in_block_pool(),
            "zero_based_block_index '{}' exceeds number_of_blocks_available_in_block_pool '{}'",
            zero_based_block_index,
            self.number_of_blocks_available_in_block_pool()
        );

        let result = unsafe { pmemblk_write(self, from, zero_based_block_index as c_longlong) };
        if likely(result == 0) {
            return;
        } else if unlikely(result != -1) {
            panic!(
                "pmemblk_write() return a value which wasn't -1 or 0, but '{}'",
                result
            );
        } else {
            PmdkError::block_panic("pmemblk_write")
        }
    }

    #[inline(always)]
    fn set_zero(self, zero_based_block_index: usize) {
        debug_assert_self_is_not_null!(self);
        debug_assert!(
            zero_based_block_index < self.number_of_blocks_available_in_block_pool(),
            "zero_based_block_index '{}' exceeds number_of_blocks_available_in_block_pool '{}'",
            zero_based_block_index,
            self.number_of_blocks_available_in_block_pool()
        );

        let result = unsafe { pmemblk_set_zero(self, zero_based_block_index as c_longlong) };
        if likely(result == 0) {
            return;
        } else if unlikely(result != -1) {
            panic!(
                "pmemblk_set_zero() return a value which wasn't -1 or 0, but '{}'",
                result
            );
        } else {
            PmdkError::block_panic("pmemblk_set_zero")
        }
    }

    #[inline(always)]
    fn set_error(self, zero_based_block_index: usize) {
        debug_assert_self_is_not_null!(self);
        debug_assert!(
            zero_based_block_index < self.number_of_blocks_available_in_block_pool(),
            "zero_based_block_index '{}' exceeds number_of_blocks_available_in_block_pool '{}'",
            zero_based_block_index,
            self.number_of_blocks_available_in_block_pool()
        );

        let result = unsafe { pmemblk_set_error(self, zero_based_block_index as c_longlong) };
        if likely(result == 0) {
            return;
        } else if unlikely(result != -1) {
            panic!(
                "pmemblk_set_error() return a value which wasn't -1 or 0, but '{}'",
                result
            );
        } else {
            PmdkError::block_panic("pmemblk_set_error")
        }
    }
}
