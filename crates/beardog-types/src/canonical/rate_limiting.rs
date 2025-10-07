//! # Canonical Rate Limiting Configuration
//!
//! This module provides the canonical rate limiting configuration used across the BearDog ecosystem.
//!
//! ## Usage
//!
//! ```rust
//! use beardog_types::canonical::config::domains::network::{
//!     RateLimitConfig, RateLimitStrategy, RateLimitScope
//! };
//!
//! let config = RateLimitConfig {
//!     enabled: true,
//!     max_requests: 100,
//!     window: std::time::Duration::from_secs(60),
//!     burst_size: Some(200),
//!     strategy: RateLimitStrategy::TokenBucket,
//!     scope: RateLimitScope::Global,
//!     allowlist: Vec::new(),
//! };
//! ```
//!
//! ## Domain-Specific Variants
//!
//! Some domains have specialized rate limiting needs:
//!
//! - **Monitoring**: `NotificationRateLimitConfig` - For notification rate limiting with per-minute and per-hour limits
//! - **Services/Endpoints**: `EndpointRateLimitConfig` - For endpoint-specific rate limiting with scoping (per-IP, per-user, etc.)
//!
//! These are kept separate because they have fundamentally different semantics from general request rate limiting.

// Re-export the canonical implementation from config::domains::network
pub use super::config::domains::network::{
    RateLimitConfig, RateLimitStrategy as RateLimitAlgorithm,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::config::domains::network::RateLimitStrategy;
    use std::time::Duration;

    #[test]
    fn test_default_rate_limit_config() {
        let config = RateLimitConfig::default();
        assert!(config.enabled);
        assert_eq!(config.max_requests, 100);
        assert_eq!(config.window, Duration::from_secs(60));
        assert_eq!(config.burst_size, Some(200));
        assert_eq!(config.strategy, RateLimitStrategy::TokenBucket);
    }
}
