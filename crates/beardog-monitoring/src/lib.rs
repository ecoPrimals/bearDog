// SPDX-License-Identifier: AGPL-3.0-only

//! # `BearDog` Monitoring and Metrics
//!
//! Comprehensive monitoring, metrics collection, and security sentinel capabilities
//! for `BearDog` applications with real-time observability and threat detection.
//!
//! ## Features
//!
//! - **Real-Time Metrics**: Performance, security, and health metrics
//! - **Security Sentinel**: Automated threat detection and response
//! - **Health Checks**: Service health monitoring and alerting
//! - **Distributed Tracing**: Request tracing across services
//! - **Alerting**: Configurable alerts and notifications
//!
//! ## Example
//!

#![cfg_attr(test, allow(clippy::expect_used))]
#![cfg_attr(test, allow(clippy::unwrap_used))]
//! ```rust
//! use beardog_monitoring::security_sentinel::{SecuritySentinel, SecuritySentinelConfig};
//! use std::collections::HashMap;
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Initialize security sentinel
//! let config = SecuritySentinelConfig::default();
//! let sentinel = SecuritySentinel::new(config);
//!
//! // Start monitoring
//! sentinel.start_monitoring()?;
//!
//! // Process security events
//! let event_data = HashMap::new();
//! sentinel.process_security_event("auth_failure", event_data).await?;
//!
//! // Get statistics
//! let stats = sentinel.get_statistics().await;
//! println!("Total events: {}", stats.total_events);
//! # Ok(())
//! # }
//! ```
//!
//! ## Architecture
//!
//! The monitoring system is built on modular components:
//! - **Metrics Collection**: Gather performance and security metrics
//! - **Security Sentinel**: Real-time threat detection
//! - **Health Monitoring**: Service health and availability
//! - **Alerting Engine**: Automated alert generation and delivery

/// Advanced metrics collection and analysis
pub mod advanced_metrics;

// NEW: Modular metrics system (replaces large advanced_metrics.rs)
/// Core metrics system
pub mod metrics;
/// Monitoring services and health checks
pub mod monitoring;
/// Security monitoring and threat detection
pub mod security_sentinel;
/// Security sentinel usage examples
pub mod security_sentinel_example;

pub use monitoring::*;
pub use security_sentinel::SecuritySentinel;

#[allow(
    unused_imports,
    clippy::float_cmp,
    clippy::absurd_extreme_comparisons,
    unused_comparisons,
    clippy::nonminimal_bool
)]
#[cfg(test)]
mod tests;
