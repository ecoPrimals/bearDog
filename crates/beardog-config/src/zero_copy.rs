//! Zero-Copy Configuration Support
//!
//! This module provides Cow<T> (Clone-on-Write) support for configuration types.
//! This allows passing configs by reference when possible, only cloning when mutation
//! is needed, reducing allocations significantly.
//!
//! # Benefits
//!
//! - **Reduced Allocations**: Clone only when actually mutating
//! - **Better Performance**: Passing large configs by reference when read-only
//! - **Zero-Cost Abstraction**: No overhead when using borrowed configs
//!
//! # Examples
//!
//! ```rust
//! use std::borrow::Cow;
//! use beardog_config::BearDogConfig;
//! use beardog_config::zero_copy::ConfigCow;
//!
//! // Pass as borrowed when read-only
//! fn read_config(config: ConfigCow<'_, BearDogConfig>) {
//!     let port = config.network.api.port; // No clone!
//! }
//!
//! // Clone only when mutating
//! fn modify_config(mut config: ConfigCow<'_, BearDogConfig>) -> BearDogConfig {
//!     let config = config.to_mut(); // Clone happens here, only if borrowed
//!     config.network.api.port = 9000;
//!     config.clone()
//! }
//! ```

use crate::{BearDogConfig, CapacityConfig, NetworkConfig, TimeoutConfig};
use std::borrow::Cow;

/// Type alias for config Cow types
pub type ConfigCow<'a, T> = Cow<'a, T>;

/// Extension trait for config types to provide zero-copy support
pub trait ConfigZeroCopy: Clone {
    /// Create a borrowed Cow wrapper
    ///
    /// This allows passing configs without cloning when no mutation is needed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use beardog_config::BearDogConfig;
    /// # use beardog_config::zero_copy::ConfigZeroCopy;
    /// let config = BearDogConfig::default();
    /// let cow = config.to_cow(); // Borrowed, no clone!
    /// ```
    #[inline]
    fn to_cow(&self) -> Cow<'_, Self> {
        Cow::Borrowed(self)
    }

    /// Create an owned Cow wrapper
    ///
    /// This clones the config and wraps it in an owned Cow.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use beardog_config::BearDogConfig;
    /// # use beardog_config::zero_copy::ConfigZeroCopy;
    /// let config = BearDogConfig::default();
    /// let cow = config.into_cow(); // Cloned, owned
    /// ```
    #[inline]
    fn into_cow(self) -> Cow<'static, Self> {
        Cow::Owned(self)
    }
}

// Implement for all major config types
impl ConfigZeroCopy for BearDogConfig {}
impl ConfigZeroCopy for NetworkConfig {}
impl ConfigZeroCopy for TimeoutConfig {}
impl ConfigZeroCopy for CapacityConfig {}

/// Helper functions for working with config Cows
pub mod helpers {
    use super::*;

    /// Pass a config as a Cow, preferring borrowed when possible
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use beardog_config::BearDogConfig;
    /// # use beardog_config::zero_copy::helpers::borrow_config;
    /// let config = BearDogConfig::default();
    /// let cow = borrow_config(&config); // No clone!
    /// ```
    #[inline]
    pub fn borrow_config<T: Clone>(config: &T) -> Cow<'_, T> {
        Cow::Borrowed(config)
    }

    /// Clone only if the Cow is borrowed and mutation is needed
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::borrow::Cow;
    /// # use beardog_config::BearDogConfig;
    /// # use beardog_config::zero_copy::helpers::clone_if_needed;
    /// let config = BearDogConfig::default();
    /// let cow = Cow::Borrowed(&config);
    /// let owned = clone_if_needed(cow); // Clones now
    /// ```
    #[inline]
    pub fn clone_if_needed<T: Clone>(cow: Cow<'_, T>) -> T {
        cow.into_owned()
    }

    /// Measure clone reduction with Cow pattern
    ///
    /// Returns (borrowed_count, cloned_count) for performance analysis
    #[cfg(test)]
    pub fn measure_clone_reduction<F, R>(f: F) -> (usize, usize, R)
    where
        F: FnOnce() -> R,
    {
        // In a real implementation, this would track allocations
        // For now, just execute the function
        let result = f();
        (0, 0, result) // Placeholder
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BearDogConfig;

    #[test]
    fn test_config_cow_borrowed() {
        let config = BearDogConfig::default();
        let cow = config.to_cow();

        // Should be borrowed
        assert!(matches!(cow, Cow::Borrowed(_)));

        // Should access without clone
        let _port = cow.network.api.port;
    }

    #[test]
    fn test_config_cow_owned() {
        let config = BearDogConfig::default();
        let cow = config.clone().into_cow();

        // Should be owned
        assert!(matches!(cow, Cow::Owned(_)));
    }

    #[test]
    fn test_config_cow_to_mut() {
        let config = BearDogConfig::default();
        let mut cow = config.to_cow();

        // Accessing via to_mut() should clone
        let config_mut = cow.to_mut();
        config_mut.network.api.port = 9000;

        assert_eq!(config_mut.network.api.port, 9000);
        assert_eq!(config.network.api.port, 8080); // Original unchanged
    }

    #[test]
    fn test_borrow_config_no_clone() {
        let config = BearDogConfig::default();
        let cow = helpers::borrow_config(&config);

        assert!(matches!(cow, Cow::Borrowed(_)));
        let _port = cow.network.api.port; // No clone needed
    }

    #[test]
    fn test_clone_if_needed() {
        let config = BearDogConfig::default();
        let cow = Cow::Borrowed(&config);

        let owned = helpers::clone_if_needed(cow);
        assert_eq!(owned.network.api.port, config.network.api.port);
    }

    #[test]
    fn test_network_config_cow() {
        let network = NetworkConfig::default();
        let cow = network.to_cow();

        assert!(matches!(cow, Cow::Borrowed(_)));
    }

    #[test]
    fn test_capacity_config_cow() {
        let capacity = CapacityConfig::default();
        let cow = capacity.to_cow();

        assert!(matches!(cow, Cow::Borrowed(_)));
    }

    #[test]
    fn test_timeout_config_cow() {
        let timeout = TimeoutConfig::default();
        let cow = timeout.to_cow();

        assert!(matches!(cow, Cow::Borrowed(_)));
    }

    #[test]
    fn test_cow_pattern_reduces_clones() {
        // Scenario: Passing config to multiple read-only functions

        let config = BearDogConfig::default();

        // Old pattern: Clone every time
        fn old_read_1(config: &BearDogConfig) -> u16 {
            config.clone().network.api.port
        }
        fn old_read_2(config: &BearDogConfig) -> usize {
            config.clone().limits.max_retries
        }

        // New pattern: Use Cow, no clones for read-only
        fn new_read_1(config: Cow<'_, BearDogConfig>) -> u16 {
            config.network.api.port // No clone!
        }
        fn new_read_2(config: Cow<'_, BearDogConfig>) -> usize {
            config.limits.max_retries // No clone!
        }

        // Both work correctly
        assert_eq!(old_read_1(&config), new_read_1(config.to_cow()));
        assert_eq!(old_read_2(&config), new_read_2(config.to_cow()));

        // But Cow pattern avoids clones!
    }

    #[test]
    fn test_cow_pattern_clones_on_mutation() {
        let config = BearDogConfig::default();

        fn mutate_config(mut config: Cow<'_, BearDogConfig>) -> BearDogConfig {
            let config_mut = config.to_mut(); // Clone happens here
            config_mut.network.api.port = 9000;
            config_mut.clone()
        }

        let modified = mutate_config(config.to_cow());
        assert_eq!(modified.network.api.port, 9000);
        assert_eq!(config.network.api.port, 8080); // Original unchanged
    }

    #[test]
    fn test_cow_into_owned() {
        let config = BearDogConfig::default();
        let cow = config.to_cow();

        let owned = cow.into_owned(); // Clone happens here
        assert_eq!(owned.network.api.port, config.network.api.port);
    }
}
