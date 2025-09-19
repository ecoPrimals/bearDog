// BearDog Monitoring and Metrics
//
// This crate provides comprehensive monitoring, metrics collection, and security
// sentinel capabilities for BearDog applications.

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
