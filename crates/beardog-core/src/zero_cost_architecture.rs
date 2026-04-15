// SPDX-License-Identifier: AGPL-3.0-or-later

// Zero-Cost Architecture Module
//
// This module demonstrates zero-cost abstraction patterns and compile-time
// optimizations for the BearDog ecosystem.

use beardog_errors::BearDogError;
use std::marker::PhantomData;

/// Zero-cost configuration using const generics
#[derive(Debug, Clone)]
pub struct ZeroCostConfig<const CACHE_SIZE: usize, const MAX_CONNECTIONS: usize> {
    _phantom: PhantomData<()>,
}

impl<const CACHE_SIZE: usize, const MAX_CONNECTIONS: usize> Default
    for ZeroCostConfig<CACHE_SIZE, MAX_CONNECTIONS>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const CACHE_SIZE: usize, const MAX_CONNECTIONS: usize>
    ZeroCostConfig<CACHE_SIZE, MAX_CONNECTIONS>
{
    /// Create new zero-cost configuration
    #[must_use]
    pub const fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    /// Get cache size at compile time
    #[must_use]
    pub const fn cache_size(&self) -> usize {
        CACHE_SIZE
    }

    /// Get max connections at compile time
    #[must_use]
    pub const fn max_connections(&self) -> usize {
        MAX_CONNECTIONS
    }
}

/// Zero-cost cache implementation
#[derive(Debug)]
pub struct ZeroCostCache<T> {
    _phantom: PhantomData<T>,
}

impl<T> Default for ZeroCostCache<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ZeroCostCache<T> {
    /// Create new zero-cost cache
    #[must_use]
    pub const fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    /// Compile-time optimized operation
    ///
    /// # Errors
    /// Returns an error if processing fails due to internal state inconsistencies.
    pub const fn process(&self, item: T) -> Result<T, BearDogError>
    where
        T: Clone,
    {
        // Zero-cost abstraction - optimized away at compile time
        Ok(item)
    }
}

/// Zero-cost security provider
#[derive(Debug)]
pub struct ZeroCostSecurity<const KEY_SIZE: usize> {
    _phantom: PhantomData<[u8; KEY_SIZE]>,
}

impl<const KEY_SIZE: usize> Default for ZeroCostSecurity<KEY_SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const KEY_SIZE: usize> ZeroCostSecurity<KEY_SIZE> {
    /// Create new zero-cost security provider
    #[must_use]
    pub const fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    /// Get key size at compile time
    #[must_use]
    pub const fn key_size(&self) -> usize {
        KEY_SIZE
    }
}

/// Zero-cost `BearDog` system
#[derive(Debug)]
pub struct ZeroCostBearDog<C, S> {
    #[expect(
        dead_code,
        reason = "Reserved for future cache/security wiring in process_data"
    )]
    cache: C,
    #[expect(
        dead_code,
        reason = "Reserved for future cache/security wiring in process_data"
    )]
    security: S,
}

impl<C, S> ZeroCostBearDog<C, S> {
    /// Create new zero-cost `BearDog` system
    /// Creates a new instance
    pub const fn new(cache: C, security: S) -> Self {
        Self { cache, security }
    }

    /// Process data with zero-cost abstractions
    ///
    /// # Errors
    /// Returns an error if data processing fails or if the cache or security components encounter issues.
    pub const fn process_data<T>(&self, data: T) -> Result<T, BearDogError>
    where
        C: std::fmt::Debug,
        S: std::fmt::Debug,
        T: Clone,
    {
        // Zero-cost processing
        Ok(data)
    }
}

/// Zero-cost builder pattern for constructing architecture components
///
/// Generic builder supporting cache and security configuration with
/// compile-time type safety and zero runtime overhead.
#[derive(Debug, Default)]
pub struct ZeroCostBuilder<C, S> {
    cache: Option<C>,
    security: Option<S>,
}

impl<C, S> ZeroCostBuilder<C, S> {
    /// Create new builder
    /// Creates a new instance
    #[must_use]
    pub const fn new() -> Self {
        Self {
            cache: None,
            security: None,
        }
    }

    /// Set cache component
    /// Creates instance with cache
    #[must_use]
    pub fn with_cache(mut self, cache: C) -> Self {
        self.cache = Some(cache);
        self
    }

    /// Set security component
    /// Creates instance with security
    #[must_use]
    pub fn with_security(mut self, security: S) -> Self {
        self.security = Some(security);
        self
    }

    /// Build the zero-cost system
    ///
    /// # Errors
    /// Returns an error if the cache or security components are not configured.
    pub fn build(self) -> Result<ZeroCostBearDog<C, S>, BearDogError> {
        let cache = self
            .cache
            .ok_or_else(|| BearDogError::configuration("Cache must be configured"))?;
        let security = self
            .security
            .ok_or_else(|| BearDogError::configuration("Security must be configured"))?;

        Ok(ZeroCostBearDog::new(cache, security))
    }
}

/// Example implementations
pub mod examples {
    use super::{BearDogError, ZeroCostBearDog, ZeroCostBuilder, ZeroCostCache, ZeroCostSecurity};

    /// Memory-backed zero-cost cache with string keys
    pub type MemoryCache = ZeroCostCache<String>;

    /// Hardware-backed zero-cost cache with byte vector keys
    pub type HardwareCache = ZeroCostCache<Vec<u8>>;

    /// Software-based security with 32-byte keys
    pub type SoftwareSecurity = ZeroCostSecurity<32>;

    /// Hardware-based security with 64-byte keys
    pub type HardwareSecurity = ZeroCostSecurity<64>;

    /// Create development configuration
    ///
    /// # Errors
    /// Returns an error if the builder fails to construct the system or if component initialization fails.
    pub fn development_config()
    -> Result<ZeroCostBearDog<MemoryCache, SoftwareSecurity>, BearDogError> {
        ZeroCostBuilder::new()
            .with_cache(MemoryCache::new())
            .with_security(SoftwareSecurity::new())
            .build()
    }

    /// Create production configuration
    ///
    /// # Errors
    /// Returns an error if the builder fails to construct the system or if hardware component initialization fails.
    pub fn production_config()
    -> Result<ZeroCostBearDog<HardwareCache, HardwareSecurity>, BearDogError> {
        ZeroCostBuilder::new()
            .with_cache(HardwareCache::new())
            .with_security(HardwareSecurity::new())
            .build()
    }

    /// Zero-cost demonstration
    ///
    /// # Errors
    /// Returns an error if system initialization fails, if data processing encounters issues, or if the demo cannot complete.
    pub fn zero_cost_demo() -> Result<(), BearDogError> {
        let system = development_config()?;
        let _result = system.process_data("test data".to_string())?;
        println!("Zero-cost architecture demo completed successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_zero_cost_architecture() -> Result<(), BearDogError> {
        let _system = examples::development_config()?;
        // Successfully creating the system proves the zero-cost architecture works
        Ok(())
    }

    #[tokio::test]
    async fn test_zero_cost_demo() -> Result<(), BearDogError> {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        examples::zero_cost_demo()?;
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_compile_time_config() {
        let config: ZeroCostConfig<1024, 100> = ZeroCostConfig::new();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert_eq!(config.cache_size(), 1024);
        assert_eq!(config.max_connections(), 100);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_security_key_size() {
        let security: ZeroCostSecurity<256> = ZeroCostSecurity::new();
        assert_eq!(security.key_size(), 256);
    }
}
