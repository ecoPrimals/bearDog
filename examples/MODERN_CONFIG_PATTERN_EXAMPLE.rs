// SPDX-License-Identifier: AGPL-3.0-only
// Modern Config Pattern Example - Best Practices
//
// This example shows how to write robust, testable, concurrent-safe
// configuration code that follows modern Rust patterns.

use serde::{Deserialize, Serialize};

// Main function for the example
fn main() {
    println!("Modern Configuration Pattern Examples");
    println!("======================================\n");

    // Example 1: Default configuration
    let limits = ResourceLimits::default();
    println!("Default limits: {limits:?}\n");

    // Example 2: Environment-based configuration
    println!("For environment-based config, set:");
    println!("  RESOURCE_MEMORY_MB=2048");
    println!("  RESOURCE_CPU_PERCENT=75");
    println!("And call ResourceLimits::from_env()\n");
}

// ============================================================================
// MODERN PATTERN: Separation of Concerns
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceLimits {
    pub memory_mb: u64,
    pub cpu_percent: u8,
    pub disk_mb: u64,
    pub network_mbps: u32,
    pub concurrent_connections: u32,
}

impl ResourceLimits {
    // ✅ GOOD: Explicit compile-time constants
    /// Default memory limit: 1GB
    pub const DEFAULT_MEMORY_MB: u64 = 1024;

    /// Default CPU limit: 50%
    pub const DEFAULT_CPU_PERCENT: u8 = 50;

    /// Default disk limit: 5GB
    pub const DEFAULT_DISK_MB: u64 = 5120;

    /// Default network limit: 100 Mbps
    pub const DEFAULT_NETWORK_MBPS: u32 = 100;

    /// Default concurrent connections: 1000
    pub const DEFAULT_CONCURRENT_CONNECTIONS: u32 = 1000;

    // ✅ GOOD: Explicit method for defaults (no env reading)
    /// Create `ResourceLimits` with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    ///
    /// # Example
    /// ```
    /// let limits = ResourceLimits::with_defaults();
    /// assert_eq!(limits.memory_mb, ResourceLimits::DEFAULT_MEMORY_MB);
    /// ```
    #[must_use]
    pub fn with_defaults() -> Self {
        Self {
            memory_mb: Self::DEFAULT_MEMORY_MB,
            cpu_percent: Self::DEFAULT_CPU_PERCENT,
            disk_mb: Self::DEFAULT_DISK_MB,
            network_mbps: Self::DEFAULT_NETWORK_MBPS,
            concurrent_connections: Self::DEFAULT_CONCURRENT_CONNECTIONS,
        }
    }

    // ✅ GOOD: Explicit method for environment-based config
    /// Create `ResourceLimits` from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    /// This makes the intent explicit and allows testing without env pollution.
    ///
    /// # Environment Variables
    /// - `BEARDOG_RESOURCE_MEMORY_MB`: Memory limit in MB (default: 1024)
    /// - `BEARDOG_RESOURCE_CPU_PERCENT`: CPU limit % (default: 50)
    /// - `BEARDOG_RESOURCE_DISK_MB`: Disk limit in MB (default: 5120)
    /// - `BEARDOG_RESOURCE_NETWORK_MBPS`: Network limit in Mbps (default: 100)
    /// - `BEARDOG_MAX_CONCURRENT_CONNECTIONS`: Connection limit (default: 1000)
    ///
    /// # Example
    /// ```
    /// let limits = ResourceLimits::from_env_get(|k| match k {
    ///     "BEARDOG_RESOURCE_MEMORY_MB" => Some("2048".to_string()),
    ///     _ => None,
    /// });
    /// assert_eq!(limits.memory_mb, 2048);
    /// ```
    #[must_use]
    pub fn from_env_get(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            memory_mb: get("BEARDOG_RESOURCE_MEMORY_MB")
                .and_then(|m| m.parse().ok())
                .unwrap_or(Self::DEFAULT_MEMORY_MB),
            cpu_percent: get("BEARDOG_RESOURCE_CPU_PERCENT")
                .and_then(|c| c.parse().ok())
                .unwrap_or(Self::DEFAULT_CPU_PERCENT),
            disk_mb: get("BEARDOG_RESOURCE_DISK_MB")
                .and_then(|d| d.parse().ok())
                .unwrap_or(Self::DEFAULT_DISK_MB),
            network_mbps: get("BEARDOG_RESOURCE_NETWORK_MBPS")
                .and_then(|n| n.parse().ok())
                .unwrap_or(Self::DEFAULT_NETWORK_MBPS),
            concurrent_connections: get("BEARDOG_MAX_CONCURRENT_CONNECTIONS")
                .and_then(|c| c.parse().ok())
                .unwrap_or(Self::DEFAULT_CONCURRENT_CONNECTIONS),
        }
    }

    /// Create `ResourceLimits` from environment variables (reads the real process environment).
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_env_get(|k| beardog_errors::process_env::var(k).ok())
    }

    // ✅ GOOD: Builder pattern for programmatic configuration
    /// Create a builder for `ResourceLimits`
    ///
    /// Allows fluent, type-safe configuration construction.
    /// Perfect for tests and programmatic configuration.
    ///
    /// # Example
    /// ```
    /// let limits = ResourceLimits::builder()
    ///     .memory_mb(4096)
    ///     .cpu_percent(75)
    ///     .build();
    /// ```
    #[must_use]
    pub fn builder() -> ResourceLimitsBuilder {
        ResourceLimitsBuilder::new()
    }
}

// ✅ GOOD: Default uses explicit defaults (deterministic, concurrent-safe)
impl Default for ResourceLimits {
    fn default() -> Self {
        Self::with_defaults()
    }
}

// ============================================================================
// BUILDER PATTERN: Type-Safe, Testable Configuration
// ============================================================================

#[derive(Debug, Clone)]
pub struct ResourceLimitsBuilder {
    memory_mb: Option<u64>,
    cpu_percent: Option<u8>,
    disk_mb: Option<u64>,
    network_mbps: Option<u32>,
    concurrent_connections: Option<u32>,
}

impl ResourceLimitsBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            memory_mb: None,
            cpu_percent: None,
            disk_mb: None,
            network_mbps: None,
            concurrent_connections: None,
        }
    }

    #[must_use]
    pub fn memory_mb(mut self, value: u64) -> Self {
        self.memory_mb = Some(value);
        self
    }

    #[must_use]
    pub fn cpu_percent(mut self, value: u8) -> Self {
        self.cpu_percent = Some(value);
        self
    }

    #[must_use]
    pub fn disk_mb(mut self, value: u64) -> Self {
        self.disk_mb = Some(value);
        self
    }

    #[must_use]
    pub fn network_mbps(mut self, value: u32) -> Self {
        self.network_mbps = Some(value);
        self
    }

    #[must_use]
    pub fn concurrent_connections(mut self, value: u32) -> Self {
        self.concurrent_connections = Some(value);
        self
    }

    #[must_use]
    pub fn build(self) -> ResourceLimits {
        ResourceLimits {
            memory_mb: self.memory_mb.unwrap_or(ResourceLimits::DEFAULT_MEMORY_MB),
            cpu_percent: self
                .cpu_percent
                .unwrap_or(ResourceLimits::DEFAULT_CPU_PERCENT),
            disk_mb: self.disk_mb.unwrap_or(ResourceLimits::DEFAULT_DISK_MB),
            network_mbps: self
                .network_mbps
                .unwrap_or(ResourceLimits::DEFAULT_NETWORK_MBPS),
            concurrent_connections: self
                .concurrent_connections
                .unwrap_or(ResourceLimits::DEFAULT_CONCURRENT_CONNECTIONS),
        }
    }
}

impl Default for ResourceLimitsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// MODERN TESTING: Isolated, Concurrent-Safe, Deterministic
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ✅ GOOD: Tests explicit defaults (no env reading)
    #[test]
    fn test_with_defaults_is_deterministic() {
        let limits1 = ResourceLimits::with_defaults();
        let limits2 = ResourceLimits::with_defaults();

        assert_eq!(limits1, limits2);
        assert_eq!(limits1.memory_mb, ResourceLimits::DEFAULT_MEMORY_MB);
        assert_eq!(limits1.cpu_percent, ResourceLimits::DEFAULT_CPU_PERCENT);
        // ✅ Fully deterministic, safe to run in parallel
    }

    // ✅ GOOD: Default implementation is deterministic
    #[test]
    fn test_default_trait_is_deterministic() {
        let limits1 = ResourceLimits::default();
        let limits2 = ResourceLimits::default();

        assert_eq!(limits1, limits2);
        // ✅ No env var reading in Default, safe for concurrent tests
    }

    // ✅ GOOD: Builder pattern test (full control, no env)
    #[test]
    fn test_builder_pattern() {
        let limits = ResourceLimits::builder()
            .memory_mb(4096)
            .cpu_percent(75)
            .build();

        assert_eq!(limits.memory_mb, 4096);
        assert_eq!(limits.cpu_percent, 75);
        // Uses defaults for unspecified fields
        assert_eq!(limits.disk_mb, ResourceLimits::DEFAULT_DISK_MB);
        // ✅ No global state, safe to run in parallel
    }

    #[test]
    fn test_from_env_get_injected_values() {
        let limits = ResourceLimits::from_env_get(|k| match k {
            "BEARDOG_RESOURCE_MEMORY_MB" => Some("2048".to_string()),
            _ => None,
        });
        assert_eq!(limits.memory_mb, 2048);
        assert_eq!(limits.cpu_percent, ResourceLimits::DEFAULT_CPU_PERCENT);
    }

    // ✅ EXCELLENT: Using temp_env for automatic cleanup
    #[cfg(feature = "temp_env")]
    #[test]
    fn test_from_env_with_temp_env() {
        temp_env::with_var("BEARDOG_RESOURCE_MEMORY_MB", Some("2048"), || {
            let limits = ResourceLimits::from_env();
            assert_eq!(limits.memory_mb, 2048);
        });
        // ✅ Env var automatically cleaned up, no pollution
    }

    // ✅ GOOD: Test constants are accessible
    #[test]
    fn test_constants_available() {
        assert_eq!(ResourceLimits::DEFAULT_MEMORY_MB, 1024);
        assert_eq!(ResourceLimits::DEFAULT_CPU_PERCENT, 50);
        // ✅ Compile-time constants, zero runtime cost
    }

    // ✅ GOOD: Test serialization/deserialization
    #[test]
    fn test_serde_roundtrip() {
        let limits = ResourceLimits::builder().memory_mb(2048).build();

        let json = serde_json::to_string(&limits).unwrap();
        let deserialized: ResourceLimits = serde_json::from_str(&json).unwrap();

        assert_eq!(limits, deserialized);
        // ✅ Config can be saved/loaded from files
    }
}

// ============================================================================
// PRODUCTION USAGE EXAMPLES
// ============================================================================

#[cfg(doc)]
mod usage_examples {
    use super::*;

    /// Example: Using defaults in production
    fn production_with_defaults() {
        let limits = ResourceLimits::default();
        // Clear intent: using hardcoded defaults
        println!("Using default limits: {:?}", limits);
    }

    /// Example: Using environment-based config
    fn production_from_env() {
        let limits = ResourceLimits::from_env();
        // Clear intent: reading from environment
        println!("Using env-based limits: {:?}", limits);
    }

    /// Example: Using programmatic config
    fn production_programmatic(memory_mb: u64, cpu_percent: u8) {
        let limits = ResourceLimits::builder()
            .memory_mb(memory_mb)
            .cpu_percent(cpu_percent)
            .build();
        // Clear intent: programmatically configured
        println!("Using custom limits: {:?}", limits);
    }

    /// Example: Loading from file
    fn production_from_file(
        config_path: &str,
    ) -> Result<ResourceLimits, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(config_path)?;
        let limits: ResourceLimits = serde_json::from_str(&json)?;
        Ok(limits)
    }

    /// Example: Mixed sources with priority
    fn production_mixed() -> ResourceLimits {
        // Try file first, then env, then defaults
        production_from_file("config.json").unwrap_or_else(|_| ResourceLimits::from_env())
    }
}

// ============================================================================
// BENEFITS SUMMARY
// ============================================================================

// ✅ Deterministic: Default always returns same values
// ✅ Testable: No global state mutation in tests
// ✅ Concurrent-safe: Tests can run in parallel
// ✅ Explicit: Clear where values come from
// ✅ Flexible: Multiple ways to create config
// ✅ Documented: Each method has clear purpose
// ✅ Type-safe: Builder prevents invalid states
// ✅ Serializable: Can save/load from files
// ✅ Modern: Follows Rust best practices

// ============================================================================
// MIGRATION GUIDE
// ============================================================================

// ❌ OLD PATTERN:
//
// impl Default for ResourceLimits {
//     fn default() -> Self {
//         Self {
//             memory_mb: beardog_errors::process_env::var("...").unwrap_or(1024),
//             // Implicit env reading, non-deterministic
//         }
//     }
// }
//
// let limits = ResourceLimits::default(); // Where do values come from?

// ✅ NEW PATTERN:
//
// impl ResourceLimits {
//     pub const DEFAULT_MEMORY_MB: u64 = 1024;
//     pub fn with_defaults() -> Self { ... }
//     pub fn from_env() -> Self { ... }
//     pub fn builder() -> Builder { ... }
// }
//
// impl Default for ResourceLimits {
//     fn default() -> Self { Self::with_defaults() }
// }
//
// let limits = ResourceLimits::from_env(); // Explicit intent!

// MIGRATION STEPS:
// 1. Add const DEFAULT_* fields
// 2. Add with_defaults() method
// 3. Add from_env() method (move env reading here)
// 4. Make Default call with_defaults()
// 5. Update tests to use specific methods
// 6. Update production code to use from_env() explicitly

// TIME ESTIMATE: 15-30 minutes per struct
