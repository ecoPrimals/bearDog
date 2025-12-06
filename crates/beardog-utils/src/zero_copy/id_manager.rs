// Safe ID management utilities
//
// This module provides ID generation and management using safe Rust patterns.

use std::sync::atomic::{AtomicU64, Ordering};
// use std::sync::Arc; // Currently unused but kept for future shared ID management

/// Thread-safe ID generator
#[derive(Debug)]
pub struct IdManager {
    counter: AtomicU64,
    prefix: String,
}

impl IdManager {
    /// Create new ID manager
    /// Creates a new instance
    #[must_use]
    pub fn new(prefix: &str) -> Self {
        Self {
            counter: AtomicU64::new(0),
            prefix: prefix.to_string(),
        }
    }

    /// Generate next ID
    pub fn next_id(&self) -> String {
        let id = self.counter.fetch_add(1, Ordering::SeqCst);
        format!("{}_{:06}", self.prefix, id)
    }

    /// Get current count
    pub fn current_count(&self) -> u64 {
        self.counter.load(Ordering::SeqCst)
    }
}

impl Default for IdManager {
    fn default() -> Self {
        Self::new("id")
    }
}

/// Global ID manager
static GLOBAL_ID_MANAGER: std::sync::OnceLock<IdManager> = std::sync::OnceLock::new();

/// Get global ID manager
pub fn global_id_manager() -> &'static IdManager {
    GLOBAL_ID_MANAGER.get_or_init(|| IdManager::new("global"))
}

/// Generate a global ID
#[must_use]
pub fn generate_id() -> String {
    global_id_manager().next_id()
}

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_manager_new() {
        let manager = IdManager::new("test");
        assert_eq!(manager.current_count(), 0);
    }

    #[test]
    fn test_id_manager_next_id() {
        let manager = IdManager::new("user");
        let id1 = manager.next_id();
        let id2 = manager.next_id();

        assert!(id1.starts_with("user_"));
        assert!(id2.starts_with("user_"));
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_id_manager_sequential() {
        let manager = IdManager::new("item");
        let id1 = manager.next_id();
        assert_eq!(id1, "item_000000");

        let id2 = manager.next_id();
        assert_eq!(id2, "item_000001");

        let id3 = manager.next_id();
        assert_eq!(id3, "item_000002");
    }

    #[test]
    fn test_id_manager_current_count() {
        let manager = IdManager::new("count");
        assert_eq!(manager.current_count(), 0);

        manager.next_id();
        assert_eq!(manager.current_count(), 1);

        manager.next_id();
        assert_eq!(manager.current_count(), 2);
    }

    #[test]
    fn test_id_manager_default() {
        let manager = IdManager::default();
        let id = manager.next_id();
        assert!(id.starts_with("id_"));
    }

    #[test]
    fn test_id_manager_empty_prefix() {
        let manager = IdManager::new("");
        let id = manager.next_id();
        assert!(id.starts_with("_"));
    }

    #[test]
    fn test_global_id_manager() {
        let manager = global_id_manager();
        let id = manager.next_id();
        assert!(id.starts_with("global_"));
    }

    #[test]
    fn test_generate_id() {
        let id1 = generate_id();
        let id2 = generate_id();

        assert!(id1.starts_with("global_"));
        assert!(id2.starts_with("global_"));
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_id_manager_thread_safety() {
        use std::sync::Arc;
        use std::thread;

        let manager = Arc::new(IdManager::new("thread"));
        let mut handles = vec![];

        // Spawn 10 threads, each generating 10 IDs
        for _ in 0..10 {
            let manager_clone = Arc::clone(&manager);
            handles.push(thread::spawn(move || {
                let mut ids = vec![];
                for _ in 0..10 {
                    ids.push(manager_clone.next_id());
                }
                ids
            }));
        }

        // Collect all IDs
        let mut all_ids = vec![];
        for handle in handles {
            let ids = handle.join().unwrap();
            all_ids.extend(ids);
        }

        // All IDs should be unique
        let unique_count = all_ids
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len();
        assert_eq!(unique_count, 100, "All IDs should be unique");
    }
}
