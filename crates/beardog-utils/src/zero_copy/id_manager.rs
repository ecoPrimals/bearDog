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
