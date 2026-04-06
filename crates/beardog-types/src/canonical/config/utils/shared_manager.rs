// SPDX-License-Identifier: AGPL-3.0-or-later

use super::types::SharedConfigStats;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};
use tracing::{debug, error};

/// Global shared configuration manager
pub(super) static SHARED_CONFIG_MANAGER: LazyLock<SharedConfigManager> =
    LazyLock::new(SharedConfigManager::new);

/// Shared configuration manager implementation
pub struct SharedConfigManager {
    configs: RwLock<HashMap<String, Arc<dyn std::any::Any + Send + Sync>>>,
}

impl SharedConfigManager {
    /// Create new shared config manager
    fn new() -> Self {
        Self {
            configs: RwLock::new(HashMap::new()),
        }
    }

    /// Get or create shared configuration
    pub fn get_or_create<T, F>(&self, key: &str, factory: F) -> Arc<T>
    where
        T: Send + Sync + 'static,
        F: FnOnce() -> T,
    {
        // Fast path: read-only check for existing config with matching type
        {
            let configs = match self.configs.read() {
                Ok(guard) => guard,
                Err(poisoned) => {
                    error!("SharedConfigManager lock poisoned on read — recovering");
                    poisoned.into_inner()
                }
            };
            if let Some(config) = configs.get(key)
                && let Ok(typed_config) = config.clone().downcast::<T>()
            {
                debug!("📋 Retrieved shared config: {}", key);
                return typed_config;
            }
        }

        // Slow path: acquire write lock, re-check, then insert atomically
        let mut configs = match self.configs.write() {
            Ok(guard) => guard,
            Err(poisoned) => {
                error!("SharedConfigManager lock poisoned on write — recovering");
                poisoned.into_inner()
            }
        };

        // Re-check under write lock to avoid TOCTOU race
        if let Some(config) = configs.get(key)
            && let Ok(typed_config) = config.clone().downcast::<T>()
        {
            return typed_config;
        }

        let config = Arc::new(factory());
        configs.insert(key.to_string(), config.clone());
        debug!("🆕 Created new shared config: {}", key);
        config
    }

    /// Remove configuration
    pub fn remove(&self, key: &str) -> bool {
        let mut configs = match self.configs.write() {
            Ok(guard) => guard,
            Err(poisoned) => {
                error!("SharedConfigManager lock poisoned - recovering with poisoned data");
                poisoned.into_inner()
            }
        };
        let removed = configs.remove(key).is_some();
        if removed {
            debug!("🗑️ Removed shared config: {}", key);
        }
        removed
    }

    /// Clear all configurations
    pub fn clear(&self) {
        let mut configs = match self.configs.write() {
            Ok(guard) => guard,
            Err(poisoned) => {
                error!("SharedConfigManager lock poisoned - recovering with poisoned data");
                poisoned.into_inner()
            }
        };
        let count = configs.len();
        configs.clear();
        debug!("🧹 Cleared {} shared configs", count);
    }

    /// Get number of configurations
    pub fn len(&self) -> usize {
        match self.configs.read() {
            Ok(guard) => guard.len(),
            Err(poisoned) => {
                error!("SharedConfigManager lock poisoned - recovering with poisoned data");
                poisoned.into_inner().len()
            }
        }
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        match self.configs.read() {
            Ok(guard) => guard.is_empty(),
            Err(poisoned) => {
                error!("SharedConfigManager lock poisoned - recovering with poisoned data");
                poisoned.into_inner().is_empty()
            }
        }
    }

    /// Get statistics
    pub fn get_stats(&self) -> SharedConfigStats {
        let len = self.len();
        SharedConfigStats {
            active_configs: len,
            memory_usage_estimate_kb: len * 8, // Rough estimate
        }
    }
}
