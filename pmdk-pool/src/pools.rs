// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.
use crate::pmemblk::BlockPool;
use crate::pmemlog::LogPool;
use crate::pmemobj::ObjectPool;
use std::collections::HashMap;

/// A root structure representing all known persistent memory pools
#[derive(Debug, Default)]
pub struct Pools {
    block_pools: HashMap<String, BlockPool>,
    log_pools: HashMap<String, LogPool>,
    object_pools: HashMap<String, ObjectPool>,
}

impl Pools {
    /// create a new Pools
    pub fn new(
        block_pools: HashMap<String, BlockPool>,
        log_pools: HashMap<String, LogPool>,
        object_pools: HashMap<String, ObjectPool>,
    ) -> Pools {
        Pools {
            block_pools: block_pools,
            log_pools: log_pools,
            object_pools: object_pools,
        }
    }

    /// Get a block pool.
    #[inline(always)]
    pub fn get_block_pool(&self, pool_name: &str) -> Option<BlockPool> {
        self.block_pools.get(pool_name).map(|pool| pool.clone())
    }

    /// Get a log pool.
    #[inline(always)]
    pub fn get_log_pool(&self, pool_name: &str) -> Option<LogPool> {
        self.log_pools.get(pool_name).map(|pool| pool.clone())
    }

    /// Get an object pool.
    #[inline(always)]
    pub fn get_object_pool(&self, pool_name: &str) -> Option<ObjectPool> {
        self.object_pools.get(pool_name).map(|pool| pool.clone())
    }
}
