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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestConfig {
        value: i32,
    }

    #[test]
    fn test_manager_creation() {
        let manager = SharedConfigManager::new();
        assert!(manager.is_empty());
        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_get_or_create_new() {
        let manager = SharedConfigManager::new();

        let config = manager.get_or_create("test", || TestConfig { value: 42 });
        assert_eq!(config.value, 42);
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_get_or_create_existing() {
        let manager = SharedConfigManager::new();

        let config1 = manager.get_or_create("test", || TestConfig { value: 42 });
        let config2 = manager.get_or_create("test", || TestConfig { value: 100 });

        // Should get the same Arc (first value)
        assert!(Arc::ptr_eq(&config1, &config2));
        assert_eq!(config2.value, 42);
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_remove_existing() {
        let manager = SharedConfigManager::new();

        let _config = manager.get_or_create("test", || TestConfig { value: 42 });
        assert_eq!(manager.len(), 1);

        let removed = manager.remove("test");
        assert!(removed);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_remove_nonexistent() {
        let manager = SharedConfigManager::new();

        let removed = manager.remove("nonexistent");
        assert!(!removed);
    }

    #[test]
    fn test_clear() {
        let manager = SharedConfigManager::new();

        let _config1 = manager.get_or_create("test1", || TestConfig { value: 1 });
        let _config2 = manager.get_or_create("test2", || TestConfig { value: 2 });
        assert_eq!(manager.len(), 2);

        manager.clear();
        assert!(manager.is_empty());
        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_multiple_configs_different_types() {
        let manager = SharedConfigManager::new();

        let config1 = manager.get_or_create("int", || 42i32);
        let config2 = manager.get_or_create("string", || "test".to_string());

        assert_eq!(*config1, 42);
        assert_eq!(*config2, "test");
        assert_eq!(manager.len(), 2);
    }

    #[test]
    fn test_default() {
        let manager = SharedConfigManager::default();
        assert!(manager.is_empty());
    }

    #[test]
    fn test_global_shared_config() {
        let manager1 = global_shared_config();
        let manager2 = global_shared_config();

        // Should be the same instance
        assert!(std::ptr::eq(manager1, manager2));
    }

    #[test]
    fn test_get_shared_config_convenience() {
        let config = get_shared_config("test_convenience", || TestConfig { value: 99 });
        assert_eq!(config.value, 99);
    }

    #[test]
    fn test_concurrent_get_or_create() {
        let manager = Arc::new(SharedConfigManager::new());
        let mut handles = vec![];

        // Spawn multiple threads trying to get the same config
        for _ in 0..10 {
            let manager_clone = Arc::clone(&manager);
            let handle = std::thread::spawn(move || {
                manager_clone.get_or_create("shared", || TestConfig { value: 42 })
            });
            handles.push(handle);
        }

        // Collect all configs
        let configs: Vec<Arc<TestConfig>> =
            handles.into_iter().map(|h| h.join().unwrap()).collect();

        // All should point to the same Arc
        for i in 1..configs.len() {
            assert!(Arc::ptr_eq(&configs[0], &configs[i]));
        }

        // Should only have created one config
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_len_and_is_empty() {
        let manager = SharedConfigManager::new();

        assert_eq!(manager.len(), 0);
        assert!(manager.is_empty());

        let _config = manager.get_or_create("test", || TestConfig { value: 1 });

        assert_eq!(manager.len(), 1);
        assert!(!manager.is_empty());
    }

    #[test]
    fn test_different_keys_different_configs() {
        let manager = SharedConfigManager::new();

        let config1 = manager.get_or_create("key1", || TestConfig { value: 1 });
        let config2 = manager.get_or_create("key2", || TestConfig { value: 2 });

        assert_eq!(config1.value, 1);
        assert_eq!(config2.value, 2);
        assert!(!Arc::ptr_eq(&config1, &config2));
    }

    #[test]
    fn test_factory_only_called_once() {
        let manager = SharedConfigManager::new();
        let call_count = Arc::new(std::sync::atomic::AtomicU32::new(0));

        let count_clone1 = Arc::clone(&call_count);
        let _config1 = manager.get_or_create("test", || {
            count_clone1.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            TestConfig { value: 42 }
        });

        let count_clone2 = Arc::clone(&call_count);
        let _config2 = manager.get_or_create("test", || {
            count_clone2.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            TestConfig { value: 100 }
        });

        // Factory should only be called once
        assert_eq!(call_count.load(std::sync::atomic::Ordering::SeqCst), 1);
    }
}
