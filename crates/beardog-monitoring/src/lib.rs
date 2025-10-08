//! # BearDog Monitoring and Metrics
//!
//! Comprehensive monitoring, metrics collection, and security sentinel capabilities
//! for BearDog applications with real-time observability and threat detection.
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
//! ```rust,no_run
//! use beardog_monitoring::{SecuritySentinel, MonitoringConfig};
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Initialize security sentinel
//! let config = MonitoringConfig::default();
//! let sentinel = SecuritySentinel::new(config)?;
//!
//! // Monitor security events
//! sentinel.monitor_authentication_attempt("user123", true).await?;
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
