// SPDX-License-Identifier: AGPL-3.0-only

// **ADVANCED METRICS SYSTEM** - Modularized Architecture
//
// This module provides a comprehensive advanced metrics system that enables
// detailed performance, security, and ecosystem monitoring through a unified,
// extensible, and type-safe interface.
//
// ## Module Organization
//
// - `types` - Core metric types, enums, and data structures
// - `config` - Configuration structures and defaults
// - `core` - Core metrics system implementation
// - `performance` - Performance analysis and monitoring
// - `security` - Security metrics and threat detection
// - `ecosystem` - Ecosystem health monitoring
// - `analysis` - Anomaly detection and trend analysis
// - `storage` - Metrics storage and retrieval

// Public API modules
pub mod analysis;
/// Configuration management
/// Configuration management
pub mod config;
/// Core functionality
/// Core functionality
pub mod core;
pub mod ecosystem;
pub mod performance;
pub mod security;
pub mod storage;
pub mod types;

// Re-export main types for backwards compatibility
pub use analysis::{AnomalyDetector, TrendAnalyzer};
pub use config::*;
pub use core::AdvancedMetricsSystem;
pub use ecosystem::EcosystemHealthMonitor;
pub use performance::PerformanceAnalyzer;
pub use security::SecurityMetrics;
pub use storage::MetricsStore;
pub use types::*;
