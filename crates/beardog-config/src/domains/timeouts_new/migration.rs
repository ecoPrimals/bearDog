//! Migration Layer for Backward Compatibility
//!
//! Re-exports all types from the new modular structure to maintain
//! compatibility with existing code using timeouts.

#[deprecated(
    since = "0.9.1",
    note = "Use `crate::domains::timeouts::TimeoutConfig` instead. This migration layer will be removed in v1.0.0"
)]
pub use super::core::TimeoutConfig;

#[deprecated(
    since = "0.9.1",
    note = "Use `crate::domains::timeouts::TimeoutConfigBuilder` instead. This migration layer will be removed in v1.0.0"
)]
pub use super::builder::TimeoutConfigBuilder;

#[deprecated(
    since = "0.9.1",
    note = "Use `crate::domains::timeouts::default_timeouts` instead. This migration layer will be removed in v1.0.0"
)]
pub use super::defaults::default_timeouts;
