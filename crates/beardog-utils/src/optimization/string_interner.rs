// SPDX-License-Identifier: AGPL-3.0-only

//! Thread-safe [`Arc<str>`] intern table with basic hit/miss statistics.

use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Mutex-backed map from `String` to weakly-counted shared slices.
pub struct StringInterner {
    strings: Arc<Mutex<HashMap<String, Arc<str>>>>,
    stats: Arc<Mutex<InternerStats>>,
}

/// Aggregate interner traffic for tests and metrics hooks.
#[derive(Debug, Clone, Default)]
pub struct InternerStats {
    /// Number of total_requests
    pub total_requests: u64,
    /// Number of cache_hits
    pub cache_hits: u64,
    /// Number of unique_strings
    pub unique_strings: usize,
    /// Number of memory_saved
    pub memory_saved: usize,
}

impl StringInterner {
    /// Create new string interner
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            strings: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(InternerStats::default())),
        }
    }

    /// Intern a string, returning a shared reference
    pub fn intern(&self, s: &str) -> Result<Arc<str>, BearDogError> {
        let mut strings = self
            .strings
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock string cache: {e}")))?;

        let mut stats = self
            .stats
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock stats: {e}")))?;

        stats.total_requests += 1;

        if let Some(interned) = strings.get(s) {
            stats.cache_hits += 1;
            stats.memory_saved += s.len();
            Ok(interned.clone())
        } else {
            let interned = Arc::<str>::from(s);
            strings.insert(s.to_string(), interned.clone());
            stats.unique_strings = strings.len();
            Ok(interned)
        }
    }

    /// Get interning statistics
    /// Gets stats
    /// Gets stats
    pub fn get_stats(&self) -> Result<InternerStats, BearDogError> {
        let stats = self
            .stats
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock stats: {e}")))?;
        Ok(stats.clone())
    }

    /// Get hit ratio
    pub fn hit_ratio(&self) -> Result<f64, BearDogError> {
        let stats = self
            .stats
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock stats: {e}")))?;

        if stats.total_requests == 0 {
            Ok(0.0)
        } else {
            #[expect(
                clippy::cast_precision_loss,
                reason = "hit ratio from request counters"
            )]
            let hits = stats.cache_hits as f64;
            #[expect(
                clippy::cast_precision_loss,
                reason = "hit ratio from request counters"
            )]
            let tot = stats.total_requests as f64;
            Ok(hits / tot)
        }
    }

    /// Get number of unique strings
    pub fn len(&self) -> Result<usize, BearDogError> {
        let strings = self
            .strings
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock string cache: {e}")))?;
        Ok(strings.len())
    }

    /// Check if interner is empty
    /// Checks if empty
    /// Checks if empty
    pub fn is_empty(&self) -> Result<bool, BearDogError> {
        Ok(self.len()? == 0)
    }

    /// Clear all interned strings
    pub fn clear(&self) -> Result<(), BearDogError> {
        let mut strings = self
            .strings
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock string cache: {e}")))?;

        let mut stats = self
            .stats
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock stats: {e}")))?;

        strings.clear();
        *stats = InternerStats::default();
        Ok(())
    }

    /// Get memory usage estimate in bytes
    pub fn memory_usage(&self) -> Result<usize, BearDogError> {
        let strings = self
            .strings
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock string cache: {e}")))?;

        let mut total = 0;
        for (key, value) in strings.iter() {
            total += key.len()
                + value.len()
                + std::mem::size_of::<String>()
                + std::mem::size_of::<Arc<str>>();
        }
        Ok(total)
    }

    /// Get memory efficiency ratio
    pub fn memory_efficiency(&self) -> Result<f64, BearDogError> {
        let stats = self
            .stats
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock stats: {e}")))?;

        let memory_used = self.memory_usage()?;
        if memory_used == 0 {
            Ok(1.0)
        } else {
            #[expect(clippy::cast_precision_loss, reason = "memory efficiency ratio")]
            let saved = stats.memory_saved as f64;
            #[expect(clippy::cast_precision_loss, reason = "memory efficiency ratio")]
            let denom = (memory_used + stats.memory_saved) as f64;
            Ok(saved / denom)
        }
    }
}

impl Default for StringInterner {
    fn default() -> Self {
        Self::new()
    }
}

/// Global string interner instance
static GLOBAL_INTERNER: std::sync::OnceLock<StringInterner> = std::sync::OnceLock::new();

/// Get the global string interner
pub fn global_interner() -> &'static StringInterner {
    GLOBAL_INTERNER.get_or_init(StringInterner::new)
}

/// Convenience function to intern a string using the global interner
pub fn intern_string(s: &str) -> Result<Arc<str>, BearDogError> {
    global_interner().intern(s)
}

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_interner_basic() -> Result<(), BearDogError> {
        let interner = StringInterner::new();

        let s1 = interner.intern("hello")?;
        let s2 = interner.intern("hello")?;

        // Should be the same Arc
        assert!(Arc::ptr_eq(&s1, &s2));

        let stats = interner.get_stats()?;
        assert_eq!(stats.total_requests, 2);
        assert_eq!(stats.cache_hits, 1);
        assert_eq!(stats.unique_strings, 1);

        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        Ok(())
    }

    #[test]
    fn test_string_interner_different_strings() -> Result<(), BearDogError> {
        let interner = StringInterner::new();

        let s1 = interner.intern("hello")?;
        let s2 = interner.intern("world")?;

        // Should be different Arcs
        assert!(!Arc::ptr_eq(&s1, &s2));
        assert_eq!(&*s1, "hello");
        assert_eq!(&*s2, "world");

        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert_eq!(interner.len()?, 2);

        Ok(())
    }

    #[test]
    fn test_string_interner_stats() -> Result<(), BearDogError> {
        let interner = StringInterner::new();

        // Intern same string multiple times
        for _ in 0..5 {
            interner.intern("test")?;
        }

        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let hit_ratio = interner.hit_ratio()?;
        assert!((hit_ratio - 0.8).abs() < f64::EPSILON); // 4 hits out of 5 requests

        Ok(())
    }

    #[test]
    fn test_global_interner() -> Result<(), BearDogError> {
        let s1 = intern_string("global_test")?;
        let s2 = intern_string("global_test")?;

        assert!(Arc::ptr_eq(&s1, &s2));
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert_eq!(&*s1, "global_test");

        Ok(())
    }

    #[test]
    fn test_interner_clear() -> Result<(), BearDogError> {
        let interner = StringInterner::new();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal

        interner.intern("test1")?;
        interner.intern("test2")?;

        assert_eq!(interner.len()?, 2);

        interner.clear()?;

        assert_eq!(interner.len()?, 0);
        assert!(interner.is_empty()?);

        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_memory_usage() -> Result<(), BearDogError> {
        let interner = StringInterner::new();

        interner.intern("short")?;
        interner.intern("a much longer string for testing")?;

        let memory = interner.memory_usage()?;
        assert!(memory > 0);

        let efficiency = interner.memory_efficiency()?;
        assert!((0.0..=1.0).contains(&efficiency));

        Ok(())
    }
}
