// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.
use crate::pmemblk::BlockPoolsConfiguration;
use crate::pmemlog::LogPoolsConfiguration;
use crate::pmemobj::ObjectPoolsConfiguration;
use crate::pools::Pools;
use libc::mode_t;
use rust_extra::unlikely;
use std::path::Path;

/// Configuration for various kinds of persistent memory pools.
/// Can be persisted and deserialized using Serde.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Configuration {
    /// Name of the pools.
    pub pools_folder_name: String,

    /// Configuration for block pools.
    pub block_pools_configuration: BlockPoolsConfiguration,

    /// Configuration for log pools.
    pub log_pools_configuration: LogPoolsConfiguration,

    /// Configuration for object pools.
    pub object_pools_configuration: ObjectPoolsConfiguration,
}

impl Default for Configuration {
    #[inline(always)]
    fn default() -> Self {
        Self {
            pools_folder_name: "pools".to_string(),
            block_pools_configuration: Default::default(),
            log_pools_configuration: Default::default(),
            object_pools_configuration: Default::default(),
        }
    }
}

impl Configuration {
    /// Default permissions. 0o600 (ie user read-write only).
    pub const DEFAULT_PERMISSIONS_FOR_POOL_SETS: mode_t = 0o600;

    /// Opens a configuration path. If the configuration's pools folder name does exist, returns a default for Pools (which contains no pools at all).
    pub fn open(&self, configuration_folder_path: &Path) -> Pools {
        let pools_folder_path = configuration_folder_path.join(&self.pools_folder_name);

        if unlikely(!pools_folder_path.exists()) {
            return Default::default();
        }

        Pools::new(
            self.block_pools_configuration.open(&pools_folder_path),
            self.log_pools_configuration.open(&pools_folder_path),
            self.object_pools_configuration.open(&pools_folder_path),
        )
    }
}
