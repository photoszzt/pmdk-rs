// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.

/// Directly accessible (`DAX`) persistent memory, eg non-volatile DIMMs.
#[derive(Debug, Clone)]
pub struct DirectlyAccessibleFileBackedMemory {
    address: *mut c_void,
    file_backed_memory_drop_wrapper: Arc<FileBackedMemoryDropWrapper>,
}

unsafe impl Send for DirectlyAccessibleFileBackedMemory {}

unsafe impl Sync for DirectlyAccessibleFileBackedMemory {}

impl<'memory> FileBackedMemory<'memory> for DirectlyAccessibleFileBackedMemory {
    type PersistOnDropT = DirectlyAccessiblePersistOnDrop<'memory>;

    // For x86-64 (4Kb for msync).
    const ALIGNMENT: usize = 64;

    const IS_PERSISTENT: bool = true;

    const SUPPORTS_EXCLUSIVE_OPEN: bool = false;

    #[inline(always)]
    fn address(&self) -> *mut c_void {
        self.address
    }

    #[inline(always)]
    fn mapped_length(&self) -> usize {
        self.file_backed_memory_drop_wrapper.mapped_length
    }

    #[inline(always)]
    fn persist_on_drop_from(&'memory self, offset: usize) -> Self::PersistOnDropT {
        DirectlyAccessiblePersistOnDrop(self.offset(offset), PhantomData)
    }

    #[inline(always)]
    fn persist_on_drop(&'memory self) -> Self::PersistOnDropT {
        DirectlyAccessiblePersistOnDrop(self.address(), PhantomData)
    }

    #[inline(always)]
    fn copy_then_persist_at_alignment_granularity(
        &self,
        offset: usize,
        length: usize,
        from: *const c_void,
    ) {
        debug_assert!(
            offset + length <= self.mapped_length(),
            "offset '{}' + length '{}' is greater than mapped length '{}'",
            offset,
            length,
            self.mapped_length()
        );
        debug_assert!(from.is_not_null(), "from must not be null");

        unsafe { pmem_memmove_persist(self.offset(offset), from, length) };
    }

    #[inline(always)]
    fn copy_nonoverlapping_then_persist_at_alignment_granularity(
        &self,
        offset: usize,
        length: usize,
        from: *const c_void,
    ) {
        debug_assert!(
            offset + length <= self.mapped_length(),
            "offset '{}' + length '{}' is greater than mapped length '{}'",
            offset,
            length,
            self.mapped_length()
        );
        debug_assert!(from.is_not_null(), "from must not be null");

        unsafe { pmem_memcpy_persist(self.offset(offset), from, length) };
    }

    #[inline(always)]
    fn write_bytes_then_persist_at_alignment_granularity(
        &self,
        offset: usize,
        count: usize,
        value: u8,
    ) {
        debug_assert!(
            offset + count <= self.mapped_length(),
            "offset '{}' + count '{}' is greater than mapped length '{}'",
            offset,
            count,
            self.mapped_length()
        );

        unsafe { pmem_memset_persist(self.offset(offset), value as i32, count) };
    }

    #[doc(hidden)]
    #[inline(always)]
    fn _open_flags(_exclusive: bool) -> FileBackedMemoryOpenFlags {
        FileBackedMemoryOpenFlags::NONE
    }

    #[doc(hidden)]
    #[inline(always)]
    fn _finish_mapping_if_memory_is_of_correct_type(
        is_persistent_memory: bool,
        is_persistent_memory_that_supports_flushing_with_persist: bool,
    ) -> bool {
        is_persistent_memory && is_persistent_memory_that_supports_flushing_with_persist
    }

    #[doc(hidden)]
    #[inline(always)]
    fn _new(address: *mut c_void, mapped_length: usize) -> Self {
        Self {
            address,
            file_backed_memory_drop_wrapper: FileBackedMemoryDropWrapper::new(
                address,
                mapped_length,
            ),
        }
    }
}

impl DirectlyAccessibleFileBackedMemory {
    /// offset and length will be adjusted to cache line size granularity
    #[inline(always)]
    pub fn persist_quickly_at_cache_line_granularity(&self, offset: usize, length: usize) {
        debug_assert!(
            offset + length <= self.mapped_length(),
            "offset '{}' + length '{}' is greater than mapped length '{}'",
            offset,
            length,
            self.mapped_length()
        );

        self.offset(offset).persist(length);
    }

    /// First 'half' of persist_quickly_at_cache_line_granularity
    #[inline(always)]
    pub fn flush(&self, offset: usize, length: usize) {
        debug_assert!(
            offset + length <= self.mapped_length(),
            "offset '{}' + length '{}' is greater than mapped length '{}'",
            offset,
            length,
            self.mapped_length()
        );

        self.offset(offset).flush(length);
    }

    /// Second 'half' of persist_quickly_at_cache_line_granularity
    #[inline(always)]
    pub fn drain_after_flush() {
        unsafe { pmem_drain() }
    }
}
