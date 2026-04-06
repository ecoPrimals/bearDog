// SPDX-License-Identifier: AGPL-3.0-or-later

//! Configuration trait interfaces
//!
//! Provides common interfaces for config families to enable polymorphic usage
//! while preserving domain-specific implementations.
//!
//! ## Overview
//!
//! Instead of forcing config consolidation (which breaks domain boundaries),
//! these traits provide shared interfaces that enable generic algorithms
//! while keeping domain-specific configs intact.
//!
//! ## Benefits
//!
//! - **Polymorphism**: Write generic code that works with any implementation
//! - **Type Safety**: Compile-time guarantees for config usage
//! - **Domain Preservation**: Keep domain-specific features intact
//! - **Easy Extension**: Add new implementations without breaking existing code
//!
//! ## Available Traits
//!
//! - `RetryStrategy` (from `retry` module) - Common interface for retry configurations ✅
//! - `TlsConfiguration` (from `tls` module) - Common interface for TLS settings ✅
//! - `TimeoutPolicy` (from `timeout` module) - Common interface for timeout configurations ✅
//! - `CacheStrategy` (from `cache` module) - Common interface for cache configurations ✅
//! - `MonitoringConfig` (from `monitoring` module) - Common interface for monitoring settings ✅
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use beardog_types::canonical::traits::retry::RetryStrategy;
//! # use beardog_types::canonical::config::domains::bootstrap::{RetryStrategy as BootstrapRetry, CoreBootstrapConfig};
//!
//! // Generic function that works with ANY RetryStrategy implementation
//! fn should_retry<S: RetryStrategy>(strategy: &S, attempt: u32) -> bool {
//!     !strategy.is_limit_reached(attempt)
//! }
//!
//! // Works with any type implementing the trait
//! # let config = CoreBootstrapConfig {
//! #     discovery_timeout_ms: 30000,
//! #     max_discovery_attempts: 5,
//! #     min_capabilities_threshold: 3,
//! #     enable_passive_listening: true,
//! #     retry_strategy: BootstrapRetry::Exponential,
//! # };
//! # fn dummy_impl<S>(_s: &S, _a: u32) -> bool { false }
//! # assert!(!dummy_impl(&config, 1));
//!
//! // Would also work with NetworkRetryConfiguration, ResilienceRetryConfig, etc.
//! ```

pub mod cache;
pub mod monitoring;
pub mod retry;
pub mod timeout;
pub mod tls;
pub mod tls_impls;

// Re-export main traits
pub use cache::{CacheStrategy, EvictionPolicy};
pub use monitoring::{MonitoringConfig, MonitoringLevel};
pub use retry::RetryStrategy;
pub use timeout::TimeoutPolicy;
pub use tls::{TlsConfiguration, TlsVersion};
