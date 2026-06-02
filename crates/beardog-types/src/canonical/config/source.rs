// SPDX-License-Identifier: AGPL-3.0-or-later

//! Configuration Source Abstraction
//!
//! Provides a modern, thread-safe approach to configuration that eliminates
//! global mutable state and test flakiness.
//!
//! # Design Philosophy
//!
//! Instead of coupling `Default` to environment reads, configuration structs accept
//! a `ConfigSource` trait. This enables:
//! - **Thread-safe testing** - No global state interference
//! - **Deterministic behavior** - Tests are reproducible
//! - **Flexible configuration** - Multiple sources (env, file, memory, etc.)
//! - **Dependency injection** - Proper separation of concerns
//!
//! # Usage
//!
//! ## Production Code
//!
//! ```rust,no_run
//! use beardog_types::canonical::config::source::{ConfigSource, EnvConfigSource};
//!
//! # struct MyConfig;
//! # impl MyConfig {
//! #     fn from_source(_source: &dyn ConfigSource) -> Result<Self, Box<dyn std::error::Error>> {
//! #         Ok(MyConfig)
//! #     }
//! # }
//! let source = EnvConfigSource::new();
//! let config = MyConfig::from_source(&source)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Test Code
//!
//! ```rust,no_run
//! use beardog_types::canonical::config::source::{ConfigSource, TestConfigSource};
//!
//! # struct MyConfig;
//! # impl MyConfig {
//! #     fn from_source(_source: &dyn ConfigSource) -> Result<Self, Box<dyn std::error::Error>> {
//! #         Ok(MyConfig)
//! #     }
//! # }
//! let mut source = TestConfigSource::new();
//! source.set("BEARDOG_API_PORT", "8080");
//! source.set("BEARDOG_ENABLE_TLS", "true");
//!
//! let config = MyConfig::from_source(&source)?;
//! // Test is thread-safe and isolated!
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! Created: November 3, 2025 - Modern configuration architecture

use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

/// Configuration source trait
///
/// Abstracts where configuration values come from, enabling:
/// - Environment variables (production)
/// - In-memory maps (testing)
/// - File-based sources (future)
/// - Remote config services (future)
///
/// # Object Safety
///
/// This trait is object-safe (dyn-compatible) to enable `CompositeConfigSource`.
/// Generic helper methods are provided as free functions instead.
pub trait ConfigSource: Send + Sync {
    /// Get a configuration value by key
    ///
    /// Returns `None` if the key doesn't exist in this source.
    fn get(&self, key: &str) -> Option<String>;

    /// Check if a key exists in this source
    fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Get a value with a default fallback
    fn get_or(&self, key: &str, default: &str) -> String {
        self.get(key).unwrap_or_else(|| default.to_string())
    }
}

/// Helper functions for parsing configuration values
///
/// These are free functions rather than trait methods to maintain object-safety.
/// Parse a configuration value as a specific type with fallback
pub fn get_parsed<T>(source: &dyn ConfigSource, key: &str, default: T) -> T
where
    T: std::str::FromStr,
{
    source
        .get(key)
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

/// Get a boolean configuration value (supports "true", "1", "yes", "on")
pub fn get_bool(source: &dyn ConfigSource, key: &str, default: bool) -> bool {
    source.get(key).map_or(default, |v| {
        matches!(v.to_lowercase().as_str(), "true" | "1" | "yes" | "on")
    })
}

/// Environment-based configuration source
///
/// Reads configuration from actual environment variables.
/// Use this in production code.
///
/// # Thread Safety
///
/// Reading environment variables is thread-safe, but setting them
/// (via `std::env::set_var`) is not. This source only reads, making
/// it safe for concurrent use.
#[derive(Debug, Clone, Default)]
pub struct EnvConfigSource;

impl EnvConfigSource {
    /// Create a new environment configuration source
    pub const fn new() -> Self {
        Self
    }
}

impl ConfigSource for EnvConfigSource {
    fn get(&self, key: &str) -> Option<String> {
        std::env::var(key).ok()
    }
}

/// Test configuration source
///
/// Thread-safe, isolated configuration source for testing.
/// Each test can have its own instance with no global state conflicts.
///
/// # Usage
///
/// ```rust
/// use beardog_types::canonical::config::source::{ConfigSource, TestConfigSource};
///
/// let mut source = TestConfigSource::new();
/// source.set("MY_KEY", "my_value");
/// assert_eq!(source.get("MY_KEY"), Some("my_value".to_string()));
/// ```
#[derive(Debug, Clone)]
pub struct TestConfigSource {
    values: Arc<RwLock<HashMap<String, String>>>,
}

impl TestConfigSource {
    /// Create a new empty test configuration source
    pub fn new() -> Self {
        Self {
            values: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a test source with initial values
    ///
    /// # Example
    ///
    /// ```rust
    /// use beardog_types::canonical::config::source::TestConfigSource;
    ///
    /// let source = TestConfigSource::with_values(vec![
    ///     ("BEARDOG_API_PORT", "8080"),
    ///     ("BEARDOG_ENABLE_TLS", "true"),
    /// ]);
    /// ```
    pub fn with_values<I, K, V>(values: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        let map = values
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();

        Self {
            values: Arc::new(RwLock::new(map)),
        }
    }

    /// Set a configuration value
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.values.write().insert(key.into(), value.into());
    }

    /// Remove a configuration value
    pub fn remove(&mut self, key: &str) {
        self.values.write().remove(key);
    }

    /// Clear all configuration values
    pub fn clear(&mut self) {
        self.values.write().clear();
    }

    /// Get the number of configured values
    pub fn len(&self) -> usize {
        self.values.read().len()
    }

    /// Check if the source is empty
    pub fn is_empty(&self) -> bool {
        self.values.read().is_empty()
    }
}

impl Default for TestConfigSource {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigSource for TestConfigSource {
    fn get(&self, key: &str) -> Option<String> {
        self.values.read().get(key).cloned()
    }
}

/// Composite configuration source
///
/// Combines multiple sources with priority ordering.
/// Earlier sources take precedence over later ones.
///
/// # Example
///
/// ```rust
/// use beardog_types::canonical::config::source::{
///     CompositeConfigSource, EnvConfigSource, TestConfigSource
/// };
///
/// let mut test_overrides = TestConfigSource::new();
/// test_overrides.set("DEBUG", "true");
///
/// // Test overrides take precedence over environment
/// let source = CompositeConfigSource::new(vec![
///     Box::new(test_overrides),
///     Box::new(EnvConfigSource::new()),
/// ]);
/// ```
pub struct CompositeConfigSource {
    sources: Vec<Box<dyn ConfigSource>>,
}

impl CompositeConfigSource {
    /// Create a new composite source from multiple sources
    ///
    /// Sources are checked in order, first match wins.
    pub fn new(sources: Vec<Box<dyn ConfigSource>>) -> Self {
        Self { sources }
    }

    /// Add a source with higher priority (checked first)
    pub fn with_priority(mut self, source: Box<dyn ConfigSource>) -> Self {
        self.sources.insert(0, source);
        self
    }

    /// Add a source with lower priority (checked last)
    pub fn with_fallback(mut self, source: Box<dyn ConfigSource>) -> Self {
        self.sources.push(source);
        self
    }
}

impl ConfigSource for CompositeConfigSource {
    fn get(&self, key: &str) -> Option<String> {
        for source in &self.sources {
            if let Some(value) = source.get(key) {
                return Some(value);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_config::env_keys;

    #[test]
    fn test_env_source_reads_actual_env() {
        let source = EnvConfigSource::new();
        assert_eq!(
            source.get("BEARDOG_TEST_VAR_UNIQUE_XYZ_NOT_SET"),
            std::env::var(env_keys::ENV_TEST_VAR_UNIQUE_XYZ_NOT_SET).ok()
        );
    }

    #[test]
    fn test_test_source_isolation() {
        // Each test source is completely isolated
        let mut source1 = TestConfigSource::new();
        let mut source2 = TestConfigSource::new();

        source1.set("KEY", "value1");
        source2.set("KEY", "value2");

        assert_eq!(source1.get("KEY"), Some("value1".to_string()));
        assert_eq!(source2.get("KEY"), Some("value2".to_string()));
    }

    #[test]
    fn test_test_source_with_values() {
        let source = TestConfigSource::with_values(vec![("KEY1", "value1"), ("KEY2", "value2")]);

        assert_eq!(source.get("KEY1"), Some("value1".to_string()));
        assert_eq!(source.get("KEY2"), Some("value2".to_string()));
        assert_eq!(source.len(), 2);
    }

    #[test]
    fn test_config_source_helpers() {
        let mut source = TestConfigSource::new();
        source.set("PORT", "8080");
        source.set("ENABLED", "true");
        source.set("TIMEOUT", "30");

        assert_eq!(get_parsed(&source, "PORT", 0), 8080);
        assert!(get_bool(&source, "ENABLED", false));
        assert_eq!(get_parsed(&source, "TIMEOUT", 0), 30);
        assert_eq!(get_parsed(&source, "MISSING", 42), 42);
    }

    #[test]
    fn test_composite_source_priority() {
        let mut high_priority = TestConfigSource::new();
        high_priority.set("KEY", "high");

        let mut low_priority = TestConfigSource::new();
        low_priority.set("KEY", "low");
        low_priority.set("OTHER", "other_value");

        let composite =
            CompositeConfigSource::new(vec![Box::new(high_priority), Box::new(low_priority)]);

        // High priority wins
        assert_eq!(composite.get("KEY"), Some("high".to_string()));
        // Falls through to low priority
        assert_eq!(composite.get("OTHER"), Some("other_value".to_string()));
    }

    #[test]
    fn test_bool_parsing() {
        let mut source = TestConfigSource::new();

        source.set("TRUE1", "true");
        source.set("TRUE2", "TRUE");
        source.set("TRUE3", "1");
        source.set("TRUE4", "yes");
        source.set("TRUE5", "on");
        source.set("FALSE1", "false");
        source.set("FALSE2", "0");
        source.set("FALSE3", "no");

        assert!(get_bool(&source, "TRUE1", false));
        assert!(get_bool(&source, "TRUE2", false));
        assert!(get_bool(&source, "TRUE3", false));
        assert!(get_bool(&source, "TRUE4", false));
        assert!(get_bool(&source, "TRUE5", false));
        assert!(!get_bool(&source, "FALSE1", true));
        assert!(!get_bool(&source, "FALSE2", true));
        assert!(!get_bool(&source, "FALSE3", true));
    }

    #[test]
    fn test_test_source_thread_safety() {
        use std::thread;

        let source = TestConfigSource::with_values(vec![("KEY", "value")]);

        // Clone for multiple threads
        let source1 = source.clone();
        let source2 = source;

        let handle1 = thread::spawn(move || {
            for _ in 0..100 {
                assert_eq!(source1.get("KEY"), Some("value".to_string()));
            }
        });

        let handle2 = thread::spawn(move || {
            for _ in 0..100 {
                assert_eq!(source2.get("KEY"), Some("value".to_string()));
            }
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
    }
}
