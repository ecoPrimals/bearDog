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
//! - [`RetryStrategy`] - Common interface for retry configurations ✅
//! - [`TlsConfiguration`] - Common interface for TLS settings ✅
//! - [`TimeoutPolicy`] - Common interface for timeout configurations ✅
//! - [`CacheStrategy`] - Common interface for cache configurations ✅
//! - [`MonitoringConfig`] - Common interface for monitoring settings (planned)
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use beardog_types::canonical::traits::RetryStrategy;
//! use beardog_types::canonical::config::domains::retry::CanonicalRetryConfig;
//!
//! // Generic function that works with ANY RetryStrategy
//! fn should_retry<S: RetryStrategy>(strategy: &S, attempt: u32) -> bool {
//!     !strategy.is_limit_reached(attempt)
//! }
//!
//! // Works with CanonicalRetryConfig
//! let config = CanonicalRetryConfig::default();
//! assert!(should_retry(&config, 1));
//!
//! // Would also work with NetworkRetryConfiguration, ResilienceRetryConfig, etc.
//! ```

pub mod cache;
pub mod retry;
pub mod tls;
pub mod tls_impls;
pub mod timeout;

// Re-export main traits
pub use cache::{CacheStrategy, EvictionPolicy};
pub use retry::RetryStrategy;
pub use tls::{TlsConfiguration, TlsVersion};
pub use timeout::TimeoutPolicy;

