// Safe shared configuration utilities
//
// This module provides shared configuration management using safe Rust patterns.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Shared configuration manager
#[derive(Debug)]
pub struct SharedConfigManager {
    configs: RwLock<HashMap<String, Arc<dyn std::any::Any + Send + Sync>>>,
}

impl SharedConfigManager {
    /// Create new shared config manager
    /// Creates a new instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            configs: RwLock::new(HashMap::new()),
        }
    }

    /// Get or create shared configuration
    /// Gets `or_create`
    /// Gets `or_create`
    pub fn get_or_create<T, F>(&self, key: &str, factory: F) -> Arc<T>
    where
        T: Send + Sync + 'static,
        F: FnOnce() -> T,
    {
        // Try to get existing config
        {
            let configs = self.configs.read().unwrap_or_else(|poisoned| {
                tracing::warn!("Shared config lock poisoned on read, recovering");
                poisoned.into_inner()
            });
            if let Some(config) = configs.get(key) {
                if let Ok(typed_config) = config.clone().downcast::<T>() {
                    return typed_config;
                }
            }
        }

        // Create new config
        let config = Arc::new(factory());
        {
            let mut configs = self.configs.write().unwrap_or_else(|poisoned| {
                tracing::warn!("Shared config lock poisoned on write, recovering");
                poisoned.into_inner()
            });
            configs.insert(key.to_string(), config.clone());
        }
        config
    }

    /// Remove configuration
    /// Removes item
    /// Removes item
    pub fn remove(&self, key: &str) -> bool {
        let mut configs = self.configs.write().unwrap_or_else(|poisoned| {
            tracing::warn!("Shared config lock poisoned on remove, recovering");
            poisoned.into_inner()
        });
        configs.remove(key).is_some()
    }

    /// Clear all configurations
    pub fn clear(&self) {
        let mut configs = self.configs.write().unwrap_or_else(|poisoned| {
            tracing::warn!("Shared config lock poisoned on clear, recovering");
            poisoned.into_inner()
        });
        configs.clear();
    }

    /// Get number of configurations
    pub fn len(&self) -> usize {
        self.configs
            .read()
            .unwrap_or_else(|poisoned| {
                tracing::warn!("Shared config lock poisoned on len, recovering");
                poisoned.into_inner()
            })
            .len()
    }

    /// Check if empty
    /// Checks if empty
    /// Checks if empty
    pub fn is_empty(&self) -> bool {
        self.configs
            .read()
            .unwrap_or_else(|poisoned| {
                tracing::warn!("Shared config lock poisoned on is_empty, recovering");
                poisoned.into_inner()
            })
            .is_empty()
    }
}

impl Default for SharedConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global shared config manager
static GLOBAL_SHARED_CONFIG: std::sync::OnceLock<SharedConfigManager> = std::sync::OnceLock::new();

/// Get global shared config manager
pub fn global_shared_config() -> &'static SharedConfigManager {
    GLOBAL_SHARED_CONFIG.get_or_init(SharedConfigManager::new)
}

/// Get or create a shared configuration
/// Gets `shared_config`
pub fn get_shared_config<T, F>(key: &str, factory: F) -> Arc<T>
where
    T: Send + Sync + 'static,
    F: FnOnce() -> T,
{
    global_shared_config().get_or_create(key, factory)
}
